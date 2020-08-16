import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {DamageDoneService} from "./damage_done";
import {DamageTakenService} from "./damage_taken";
import {HealDoneService} from "./heal_done";
import {HealTakenService} from "./heal_taken";
import {HealMode} from "../../../domain_value/heal_mode";
import {ThreatDoneService} from "./threat_done";
import {DeathService} from "./death";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";

@Injectable({
    providedIn: "root",
})
export class RaidMeterService implements OnDestroy {

    private subscription_data: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number] | DeathOverviewRow>]>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());
    private units$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());

    private current_selection: number = -1;

    constructor(
        private damageDoneService: DamageDoneService,
        private damageTakenService: DamageTakenService,
        private healDoneService: HealDoneService,
        private healTakenService: HealTakenService,
        private threatDoneService: ThreatDoneService,
        private deathService: DeathService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_data?.unsubscribe();
    }

    get data(): Observable<Array<[number, Array<[number, number] | DeathOverviewRow>]>> {
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
                this.subscription_data = this.damageDoneService.get_data(this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 2:
                this.subscription_data = this.damageTakenService.get_data(this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 3:
                this.subscription_data = this.healDoneService.get_data(HealMode.Total, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 4:
                this.subscription_data = this.healTakenService.get_data(HealMode.Total, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 5:
                this.subscription_data = this.healDoneService.get_data(HealMode.Effective, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 6:
                this.subscription_data = this.healTakenService.get_data(HealMode.Effective, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 7:
                this.subscription_data = this.healDoneService.get_data(HealMode.Overheal, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 8:
                this.subscription_data = this.healTakenService.get_data(HealMode.Overheal, this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 9:
                this.subscription_data = this.threatDoneService.get_data(this.abilities$.getValue(), this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
            case 10:
                this.subscription_data = this.deathService.get_data(this.units$.getValue())
                    .subscribe(data => this.commit(data));
                break;
        }

        this.current_selection = selection;
    }

    private commit(data: Array<[number, Array<[number, number] | DeathOverviewRow>]>): void {
        this.abilities$.next(this.abilities$.getValue());
        this.units$.next(this.units$.getValue());
        this.data$.next(data);
    }
}
