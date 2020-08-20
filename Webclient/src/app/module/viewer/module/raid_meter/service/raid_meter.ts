import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {HealMode} from "../../../domain_value/heal_mode";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {MeterDamageService} from "./meter_damage";
import {MeterHealService} from "./meter_heal";
import {MeterThreatService} from "./meter_threat";
import {MeterDeathService} from "./meter_death";
import {MeterDispelService} from "./meter_dispel";

@Injectable({
    providedIn: "root",
})
export class RaidMeterService implements OnDestroy {

    private subscription_data: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number] | DeathOverviewRow | UnAuraOverviewRow>]>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());
    private units$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());

    private current_selection: number = -1;

    constructor(
        private meter_damage_service: MeterDamageService,
        private meter_heal_service: MeterHealService,
        private meter_threat_service: MeterThreatService,
        private meter_death_service: MeterDeathService,
        private meter_dispel_service: MeterDispelService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_data?.unsubscribe();
    }

    get data(): Observable<Array<[number, Array<[number, number] | DeathOverviewRow | UnAuraOverviewRow>]>> {
        return this.data$.asObservable();
    }

    get abilities(): Observable<Map<number, RaidMeterSubject>> {
        return this.abilities$.asObservable();
    }

    get units(): Observable<Map<number, RaidMeterSubject>> {
        return this.units$.asObservable();
    }

    select(selection: number): void {
        if (this.current_selection === selection)
            return;

        this.subscription_data?.unsubscribe();

        switch (selection) {
            case 1:
            case 2:
                this.subscription_data = this.meter_damage_service.get_data(selection === 2, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 3:
            case 4:
                this.subscription_data = this.meter_heal_service.get_data(HealMode.Total, selection === 4, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 5:
            case 6:
                this.subscription_data = this.meter_heal_service.get_data(HealMode.Effective, selection === 6, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 7:
            case 8:
                this.subscription_data = this.meter_heal_service.get_data(HealMode.Overheal, selection === 8, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 9:
            case 10:
                this.subscription_data = this.meter_threat_service.get_data(selection === 10, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 11:
            case 12:
                this.subscription_data = this.meter_death_service.get_data(selection === 12, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 13:
            case 14:
                this.subscription_data = this.meter_dispel_service.get_data(selection === 14, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
        }

        this.current_selection = selection;
    }

    private commit(data: Array<[number, Array<[number, number] | DeathOverviewRow | UnAuraOverviewRow>]>): void {
        this.abilities$.next(this.abilities$.getValue());
        this.units$.next(this.units$.getValue());
        this.data$.next(data);
    }
}
