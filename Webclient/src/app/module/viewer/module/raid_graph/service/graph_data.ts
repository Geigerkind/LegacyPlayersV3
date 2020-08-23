import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {DataSet, is_event_data_set} from "../domain_value/data_set";
import {RaidGraphKnecht} from "../tool/raid_graph_knecht";

@Injectable({
    providedIn: "root",
})
export class GraphDataService implements OnDestroy {

    private subscription: Subscription;
    private data_points$: BehaviorSubject<[Array<number>, Map<DataSet, [Array<number>, Array<number>]>]> = new BehaviorSubject([[], new Map()]);

    constructor(
        private instanceDataService: InstanceDataService
    ) {
        this.subscription = this.instanceDataService.knecht_updates.subscribe(knecht_updates => {
            for (const data_set of [...this.data_points$.getValue()[1].keys()])
                this.add_data_set(data_set);
        });
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get data_points(): Observable<[Array<number>, Map<DataSet, [Array<number>, Array<number>]>]> {
        return this.data_points$.asObservable();
    }

    update(): void {
        this.data_points$.next(this.data_points$.getValue());
    }

    async add_data_set(data_set: DataSet): Promise<void> {
        switch (data_set) {
            case DataSet.DamageDone:
            case DataSet.DamageTaken:
            case DataSet.ThreatDone:
            case DataSet.ThreatTaken:
                this.commit_data_set(data_set, RaidGraphKnecht.squash([
                    ...await this.instanceDataService.knecht_melee.graph_data_set(data_set),
                    ...await this.instanceDataService.knecht_spell.graph_data_set(data_set)
                ].sort((left, right) => left[0] - right[0])));
                break;
            case DataSet.Deaths:
            case DataSet.Kills:
                this.commit_data_set(data_set, [
                    ...await this.instanceDataService.knecht_melee.graph_data_set(data_set),
                    ...await this.instanceDataService.knecht_spell.graph_data_set(data_set)
                ].sort((left, right) => left[0] - right[0]));
                break;
            case DataSet.TotalHealingDone:
            case DataSet.TotalHealingTaken:
            case DataSet.EffectiveHealingDone:
            case DataSet.EffectiveHealingTaken:
            case DataSet.OverhealingDone:
            case DataSet.OverhealingTaken:
            case DataSet.DispelsDone:
            case DataSet.DispelsReceived:
            case DataSet.InterruptDone:
            case DataSet.InterruptReceived:
            case DataSet.SpellStealDone:
            case DataSet.SpellStealReceived:
                this.commit_data_set(data_set, await this.instanceDataService.knecht_spell.graph_data_set(data_set));
                break;
        }
    }

    commit_data_set(data_set: DataSet, data_set_points: Array<[number, number]>): void {
        const [old_total_x_axis, data_points] = this.data_points$.getValue();
        const x_axis = new Array<number>();
        const y_axis = [];
        for (const [x, y] of data_set_points) {
            x_axis.push(x);
            y_axis.push(y);
        }

        data_points.set(data_set, [x_axis, y_axis]);
        this.compute_event_y_values(data_points);

        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    remove_data_set(data_set: DataSet): void {
        const [old_x_axis, data_points] = this.data_points$.getValue();
        data_points.delete(data_set);
        this.compute_event_y_values(data_points);
        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    private compute_event_y_values(data_points: Map<DataSet, [Array<number>, Array<number>]>): void {
        const max_value = [...data_points.entries()].filter(([data_set, points]) => !is_event_data_set(data_set))
            .map(([set, [x, y]]) => y)
            .reduce((acc, y) => Math.max(acc, ...y), 0) * 0.75;
        for (const [c_data_set, [c_x_axis, c_yaxis]] of data_points) {
            if (is_event_data_set(c_data_set)) {
                for (let i = 0; i < c_yaxis.length; ++i)
                    c_yaxis[i] = max_value;
            }
        }
    }

    private compute_x_axis(data_points: Map<DataSet, [Array<number>, Array<number>]>): Array<number> {
        let result = new Set<number>();
        for (const [timestamps, values] of data_points.values())
            result = new Set<number>([...result.values(), ...timestamps.values()]);
        return [...result.values()].sort((left, right) => left - right);
    }
}
