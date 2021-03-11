import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subject, Subscription, zip} from "rxjs";
import {DataSet, is_event_data_set} from "../domain_value/data_set";
import {RaidGraphKnecht} from "../tool/raid_graph_knecht";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import {MeterAuraUptimeService} from "../../raid_meter/service/meter_aura_uptime";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {flatten_aura_uptime_to_spell_map} from "../../raid_meter/stdlib/aura_uptime";
import {SpellService} from "../../../service/spell";
import {map} from "rxjs/operators";
import {UtilService} from "../../raid_meter/service/util";
import {get_absorb_data_points} from "../../../stdlib/absorb";
import {ChartType, number_to_chart_type} from "../domain_value/chart_type";

@Injectable({
    providedIn: "root",
})
export class GraphDataService implements OnDestroy {
    private static readonly X_AXIS_RESOLUTION: number = 1000;

    private subscription_absorb_done: Subscription;

    private subscription: Subscription;
    private data_points$: BehaviorSubject<[Array<number>, Array<[DataSet, [Array<number>, Array<number>]]>]> = new BehaviorSubject([[], []]);
    private current_data: Map<DataSet, [Array<number>, Array<number>]> = new Map();

    private UNUSED_abilities$: Map<number, RaidMeterSubject> = new Map();
    private UNUSED_units$: Map<number, RaidMeterSubject> = new Map();

    private source_ability_intervals: Array<[number, Array<[number, number]>]> = [];
    private target_ability_intervals: Array<[number, Array<[number, number]>]> = [];

    private source_abilities$: Subject<Array<{ id: number, label: string }>> = new Subject();
    private target_abilities$: Subject<Array<{ id: number, label: string }>> = new Subject();

    private source_meter_aura_uptime_service: MeterAuraUptimeService;
    private target_meter_aura_uptime_service: MeterAuraUptimeService;

    // Helper for Export
    private graph_mode: ChartType = ChartType.Line;
    private selectedSourceAbilities: Array<number> = [];
    private selectedTargetAbilities: Array<number> = [];
    public overwrite_selection: Subject<any> = new Subject();

    constructor(
        private instanceDataService: InstanceDataService,
        private spell_service: SpellService,
        private util_service: UtilService
    ) {
        this.subscription = this.instanceDataService.knecht_updates.subscribe(([knecht_updates, _]) => {
            if (knecht_updates.some(elem => [KnechtUpdates.NewData, KnechtUpdates.FilterChanged].includes(elem))) {
                for (const [data_set, data] of this.data_points$.getValue()[1])
                    this.add_data_set(data_set);
            }
        });

        this.source_meter_aura_uptime_service = new MeterAuraUptimeService(instanceDataService, util_service);
        this.target_meter_aura_uptime_service = new MeterAuraUptimeService(instanceDataService, util_service);

        this.subscription.add(this.source_meter_aura_uptime_service.get_data(false, this.UNUSED_abilities$, this.UNUSED_units$).subscribe(data => {
            this.source_ability_intervals = flatten_aura_uptime_to_spell_map(data);
            const observables = this.source_ability_intervals
                .map(([spell_id, intervals]) => this.spell_service.get_localized_basic_spell(spell_id)
                    .pipe(map(spell => {
                        return {
                            id: spell_id,
                            label: spell?.localization
                        };
                    })));
            zip(...observables).subscribe(abilities => this.source_abilities$.next(abilities));
        }));

        this.subscription.add(this.target_meter_aura_uptime_service.get_data(true, this.UNUSED_abilities$, this.UNUSED_units$).subscribe(data => {
            this.target_ability_intervals = flatten_aura_uptime_to_spell_map(data);
            const observables = this.target_ability_intervals
                .map(([spell_id, intervals]) => this.spell_service.get_localized_basic_spell(spell_id)
                    .pipe(map(spell => {
                        return {
                            id: spell_id,
                            label: spell?.localization
                        };
                    })));
            zip(...observables).subscribe(abilities => this.target_abilities$.next(abilities));
        }));
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    set_graph_mode(chart_type: number): void {
        this.graph_mode = number_to_chart_type(chart_type);
    }

    get_current_graph_mode(): ChartType {
        return this.graph_mode;
    }

    get_selected_data_sets(): Array<DataSet> {
        return [...this.current_data.keys()].filter(dataset => !is_event_data_set(dataset));
    }

    get_selected_events(): Array<DataSet> {
        return [...this.current_data.keys()].filter(dataset => is_event_data_set(dataset));
    }

    setSelectedSourceAbilities(abilities: Array<number>): void {
        this.selectedSourceAbilities = [...abilities];
    }

    get_selected_source_auras(): Array<number> {
        return this.selectedSourceAbilities;
    }

    setSelectedTargetAbilities(abilities: Array<number>): void {
        this.selectedTargetAbilities = [...abilities];
    }

    get_selected_target_auras(): Array<number> {
        return this.selectedTargetAbilities;
    }

    get data_points(): Observable<[Array<number>, Array<[DataSet, [Array<number>, Array<number>]]>]> {
        return this.data_points$.asObservable();
    }

    get source_abilities(): Observable<Array<{ id: number, label: string }>> {
        return this.source_abilities$.asObservable();
    }

    get target_abilities(): Observable<Array<{ id: number, label: string }>> {
        return this.target_abilities$.asObservable();
    }

    // TODO: This causes many updates
    get_source_aura_uptime_interval(ability_id: number): Array<[number, number]> {
        const result = this.source_ability_intervals
            .find(([spell_id, interval]) => spell_id === ability_id);
        return !!result ? result[1] : undefined;
    }

    get_target_aura_uptime_interval(ability_id: number): Array<[number, number]> {
        const result = this.target_ability_intervals
            .find(([spell_id, interval]) => spell_id === ability_id);
        return !!result ? result[1] : undefined;
    }

    update(): void {
        this.data_points$.next(this.data_points$.getValue());
    }

    async add_data_set(data_set: DataSet): Promise<void> {
        if (!this.instanceDataService.isInitialized()) return;
        switch (data_set) {
            case DataSet.DamageDone:
            case DataSet.DamageTaken:
                this.commit_data_set(data_set, RaidGraphKnecht.squash([
                    ...await this.instanceDataService.knecht_melee.graph_data_set(data_set),
                    ...await this.instanceDataService.knecht_spell_damage.graph_data_set(data_set)
                ].sort((left, right) => left[0] - right[0])));
                break;
            case DataSet.ThreatDone:
            case DataSet.ThreatTaken:
                // TODO
                break;
            case DataSet.Deaths:
            case DataSet.Kills:
                this.commit_data_set(data_set, await this.instanceDataService.knecht_melee.graph_data_set(data_set));
                break;
            case DataSet.TotalHealingDone:
            case DataSet.TotalHealingTaken:
            case DataSet.EffectiveHealingDone:
            case DataSet.EffectiveHealingTaken:
            case DataSet.OverhealingDone:
            case DataSet.OverhealingTaken:
                this.commit_data_set(data_set, await this.instanceDataService.knecht_heal.graph_data_set(data_set));
                break;
            case DataSet.DispelsDone:
            case DataSet.DispelsReceived:
            case DataSet.SpellStealDone:
            case DataSet.SpellStealReceived:
            case DataSet.InterruptDone:
            case DataSet.InterruptReceived:
                this.commit_data_set(data_set, await this.instanceDataService.knecht_un_aura.graph_data_set(data_set));
                break;
            case DataSet.AbsorbDone:
            case DataSet.AbsorbTaken:
                this.commit_data_set(data_set, RaidGraphKnecht.squash((await get_absorb_data_points(data_set === DataSet.AbsorbTaken, this.instanceDataService))
                    .reduce((acc, [subject_id, [subject, points]]) => [...acc, ...points.map(([spell_id, ts, amount]) => [ts, amount])], [])
                    .sort((left, right) => left[0] - right[0])));
                break;
            case DataSet.HealAndAbsorbDone:
            case DataSet.HealAndAbsorbTaken:
                this.commit_data_set(data_set, RaidGraphKnecht.squash([...(await get_absorb_data_points(data_set === DataSet.HealAndAbsorbTaken, this.instanceDataService))
                    .reduce((acc, [subject_id, [subject, points]]) => [...acc, ...points.map(([spell_id, ts, amount]) => [ts, amount])], []),
                    ...await this.instanceDataService.knecht_heal.graph_data_set(data_set === DataSet.HealAndAbsorbDone ? DataSet.EffectiveHealingDone : DataSet.EffectiveHealingTaken)]
                    .sort((left, right) => left[0] - right[0])));
                break;
        }
    }

    commit_data_set(data_set: DataSet, data_set_points: Array<[number, number]>): void {
        console.log(data_set, data_set_points);
        const data_points = this.current_data;
        const x_axis = new Array<number>();
        const y_axis = [];
        for (const [x, y] of data_set_points) {
            x_axis.push(x);
            y_axis.push(y);
        }

        data_points.set(data_set, [x_axis, y_axis]);
        const res_x_axis = this.compute_x_axis();
        const res_xy_data_sets = this.compute_dataset_xy_axis(res_x_axis);
        this.data_points$.next([res_x_axis, res_xy_data_sets]);
    }

    remove_data_set(data_set: DataSet): void {
        this.current_data.delete(data_set);
        const res_x_axis = this.compute_x_axis();
        const res_xy_data_sets = this.compute_dataset_xy_axis(res_x_axis);
        this.data_points$.next([res_x_axis, res_xy_data_sets]);

        switch (data_set) {
            case DataSet.AbsorbDone:
                this.subscription_absorb_done?.unsubscribe();
                break;
        }
    }

    private compute_dataset_xy_axis(normalized_x_axis: Array<number>): Array<[DataSet, [Array<number>, Array<number>]]> {
        const data_points = this.current_data;
        const max_value = [...data_points.entries()].filter(([data_set, points]) => !is_event_data_set(data_set))
            .map(([set, [x, y]]) => y)
            .reduce((acc, y) => Math.max(acc, ...y), 0) * 0.75;

        if (normalized_x_axis.length < 2)
            return [...data_points.entries()];

        const result = [];
        for (const [data_set, [timestamps, values]] of data_points.entries()) {
            const last_ts = normalized_x_axis[0];
            let current_timestamps_index = 0;
            let current_timestamps_ts = timestamps[0];
            const data_set_res = [];
            for (let i = 1; i < normalized_x_axis.length; ++i) {
                const current_ts = normalized_x_axis[i];
                while (current_timestamps_index < timestamps.length) {
                    if (current_ts < current_timestamps_ts)
                        break;
                    if (Math.abs(current_timestamps_ts - current_ts) < Math.abs(current_timestamps_ts - last_ts))
                        data_set_res.push(current_ts);
                    else data_set_res.push(last_ts);
                    ++current_timestamps_index;
                    current_timestamps_ts = timestamps[current_timestamps_index];
                }
                if (current_timestamps_index === timestamps.length)
                    break;
            }
            const res_values = [...values];
            if (is_event_data_set(data_set)) {
                for (let i = 0; i < res_values.length; ++i)
                    res_values[i] = max_value;
            }
            result.push([data_set, [data_set_res, res_values]]);
        }
        return result;
    }

    private compute_x_axis(): Array<number> {
        const data_points = this.current_data;
        let result = new Set<number>();
        for (const [timestamps, values] of data_points.values())
            result = new Set<number>([...result.values(), ...timestamps]);

        const intermediate_result = [...result.values()].sort((left, right) => left - right);
        if (intermediate_result.length < 2)
            return intermediate_result;

        const graph_resolution = Math.min(GraphDataService.X_AXIS_RESOLUTION, intermediate_result.length) + 1;
        const min = Math.min(...intermediate_result);
        const max = Math.max(...intermediate_result);
        const final_result = [];
        const tick_size = (max - min) / graph_resolution;
        for (let ts = min; ts <= max; ts += tick_size)
            final_result.push(ts);
        return final_result;
    }
}
