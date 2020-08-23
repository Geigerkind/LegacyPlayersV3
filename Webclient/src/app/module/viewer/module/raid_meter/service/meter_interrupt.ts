import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {get_unit_id} from "../../../domain_value/unit";

@Injectable({
    providedIn: "root",
})
export class MeterInterruptService implements OnDestroy {

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<UnAuraOverviewRow>]>> = new BehaviorSubject([]);
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

    get_data(mode: boolean, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<UnAuraOverviewRow>]>> {
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
        const result = new Map<number, Array<UnAuraOverviewRow>>();

        const result1 = await this.instanceDataService.knecht_spell.meter_interrupt(this.current_mode);
        for (const [subject_id, [subject, un_aura_overview_rows]] of result1) {
            if (!result.has(subject_id))
                result.set(subject_id, un_aura_overview_rows);
            else result.get(subject_id).push(...un_aura_overview_rows);
            for (const row of un_aura_overview_rows) {
                const caster_id = get_unit_id(row.caster);
                const target_id = get_unit_id(row.target);
                if (!this.units$.has(caster_id))
                    this.units$.set(caster_id, this.utilService.get_row_unit_subject(row.caster));
                if (!this.units$.has(target_id))
                    this.units$.set(target_id, this.utilService.get_row_unit_subject(row.target));
                if (!this.abilities$.has(row.caster_spell_id))
                    this.abilities$.set(row.caster_spell_id, this.utilService.get_row_ability_subject(row.caster_spell_id));
                if (!this.abilities$.has(row.target_spell_id))
                    this.abilities$.set(row.target_spell_id, this.utilService.get_row_ability_subject(row.target_spell_id));
            }
        }

        // @ts-ignore
        this.data$.next([...result.entries()]);
    }
}
