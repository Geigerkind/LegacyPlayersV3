import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {DamageDoneService} from "./damage_done";

@Injectable({
    providedIn: "root",
})
export class RaidMeterService implements OnDestroy {

    private subscription_data: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());
    private units$: BehaviorSubject<Map<number, RaidMeterSubject>> = new BehaviorSubject(new Map());

    private current_selection: number = -1;

    constructor(
        private damageDoneService: DamageDoneService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_data?.unsubscribe();
    }

    get data(): Observable<Array<[number, Array<[number, number]>]>> {
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
        }

        this.current_selection = selection;
    }

    private commit(data: Array<[number, Array<[number, number]>]>): void {
        this.abilities$.next(this.abilities$.getValue());
        this.units$.next(this.units$.getValue());
        this.data$.next(data);
    }
}
