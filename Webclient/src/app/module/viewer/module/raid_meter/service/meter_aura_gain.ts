import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import {AuraGainOverviewRow} from "../domain_value/aura_gain_overview_row";
import {get_unit_id} from "../../../domain_value/unit";

@Injectable({
    providedIn: "root",
})
export class MeterAuraGainService implements OnDestroy {

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<AuraGainOverviewRow>]>> = new BehaviorSubject([]);
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

    get_data(mode: boolean, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<AuraGainOverviewRow>]>> {
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
        if (!this.instanceDataService.isInitialized()) return;
        const data_set = await this.instanceDataService.knecht_aura.meter_aura_gain(this.current_mode);
        const result = [];
        for (const [subject_id, [subject, rows]] of data_set) {
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(subject));
            for (const row of rows) {
                if (!this.abilities$.has(row.ability))
                    this.abilities$.set(row.ability, this.utilService.get_row_ability_subject(row.ability));
                const target_id = get_unit_id(row.target, false);
                if (!this.units$.has(target_id))
                    this.units$.set(target_id, this.utilService.get_row_unit_subject(row.target));
            }
            result.push([subject_id, rows]);
        }

        this.data$.next(result);
    }
}
