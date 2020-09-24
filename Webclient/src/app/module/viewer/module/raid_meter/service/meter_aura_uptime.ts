import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";

@Injectable({
    providedIn: "root",
})
export class MeterAuraUptimeService implements OnDestroy {

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, Array<[number, number]>]>]>> = new BehaviorSubject([]);
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

    get_data(mode: boolean, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<[number, Array<[number, number]>]>]>> {
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
            if (knecht_update.includes(KnechtUpdates.FilterChanged) || (knecht_update.includes(KnechtUpdates.NewData) && [6].some(evt => evt_types.includes(evt))))
                this.merge_data();
        });
    }

    private async merge_data(): Promise<void> {
        const result = new Map<number, Map<number, Array<[number, number]>>>();

        const result1 = await this.instanceDataService.knecht_aura.meter_aura_uptime(this.current_mode);
        for (const [subject_id, [subject, abilities]] of result1) {
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(subject));
            if (!result.has(subject_id))
                result.set(subject_id, new Map());
            const ability_map = result.get(subject_id);
            for (const [ability_id, intervals] of abilities) {
                if (!this.abilities$.has(ability_id))
                    this.abilities$.set(ability_id, this.utilService.get_row_ability_subject(ability_id));
                // @ts-ignore
                ability_map.set(ability_id, intervals);
            }
        }

        this.data$.next([...result.entries()].map(([subject_id, abilities]) => [subject_id, [...abilities.entries()]]));
    }
}
