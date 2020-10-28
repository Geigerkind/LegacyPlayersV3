import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import {get_absorb_data_points} from "../../../stdlib/absorb";

@Injectable({
    providedIn: "root",
})
export class MeterAbsorbService implements OnDestroy {
    private static readonly ABSORBING_SPELL_IDS: Array<number> = [25218];

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;
    private current_mode: boolean = false;

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get_data(mode: boolean, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<[number, number]>]>> {
        if (!this.initialized) {
            this.abilities$ = abilities;
            this.units$ = units;
            this.current_mode = mode;
            this.initialize();
        } else if (this.current_mode !== mode) {
            this.current_mode = mode;
            this.merge_data();
        }
        return this.data$.asObservable();
    }

    private initialize(): void {
        this.initialized = true;
        this.merge_data();
        this.subscription = this.instanceDataService.knecht_updates.subscribe(async ([knecht_update, evt_types]) => {
            if (knecht_update.includes(KnechtUpdates.FilterChanged) || (knecht_update.includes(KnechtUpdates.NewData) && [6, 12, 13].some(evt => evt_types.includes(evt))))
                this.merge_data();
        });
    }

    private async merge_data(): Promise<void> {
        if (!this.instanceDataService.isInitialized()) return;
        const result = new Map<number, Map<number, number>>();

        const data_points = await get_absorb_data_points(this.current_mode, this.instanceDataService);

        for (const [subject_id, [subject, points]] of data_points) {
            for (const [absorb_spell_id, timestamp, amount] of points) {
                if (!this.units$.has(subject_id))
                    this.units$.set(subject_id, this.utilService.get_row_unit_subject(subject));
                if (!result.has(subject_id))
                    result.set(subject_id, new Map());
                const ability_map = result.get(subject_id);
                if (!this.abilities$.has(absorb_spell_id))
                    this.abilities$.set(absorb_spell_id, this.utilService.get_row_ability_subject(absorb_spell_id));

                if (ability_map.has(absorb_spell_id)) ability_map.set(absorb_spell_id, ability_map.get(absorb_spell_id) + amount);
                else ability_map.set(absorb_spell_id, amount);
            }
        }

        this.data$.next([...result.entries()].map(([subject_id, abilities]) => [subject_id, [...abilities.entries()]]));
    }
}
