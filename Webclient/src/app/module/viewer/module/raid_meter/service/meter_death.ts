import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";

@Injectable({
    providedIn: "root",
})
export class MeterDeathService implements OnDestroy {

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<DeathOverviewRow>]>> = new BehaviorSubject([]);
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

    get_data(mode: boolean, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<DeathOverviewRow>]>> {
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
        this.subscription = this.instanceDataService.knecht_updates.subscribe(async () => this.merge_data());
    }

    private async merge_data(): Promise<void> {
        const result = new Map<number, Array<DeathOverviewRow>>();

        const result1 = await this.instanceDataService.knecht_melee.meter_death(this.current_mode);
        const result2 = await this.instanceDataService.knecht_spell.meter_death(this.current_mode);
        for (const data_set of [result1, result2]) {
            for (const [subject_id, [subject, death_overview_rows]] of data_set) {
                if (!this.units$.has(subject_id))
                    this.units$.set(subject_id, this.utilService.get_row_unit_subject(subject));
                if (!result.has(subject_id))
                    result.set(subject_id, death_overview_rows);
                else result.get(subject_id).push(...death_overview_rows);
                for (const row of death_overview_rows) {
                    if (!this.abilities$.has(row.killing_blow.ability_id))
                        this.abilities$.set(row.killing_blow.ability_id, this.utilService.get_row_ability_subject(row.killing_blow.ability_id));
                }
            }
        }

        this.data$.next([...result.entries()]);
    }
}
