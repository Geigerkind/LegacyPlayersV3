import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {MeterHealService} from "./meter_heal";
import {MeterAbsorbService} from "./meter_absorb";
import {HealMode} from "../../../domain_value/heal_mode";

@Injectable({
    providedIn: "root",
})
export class MeterHealAndAbsorbService implements OnDestroy {

    private subscription: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;
    private current_mode: boolean = false;

    private heal_data: Array<[number, Array<[number, number]>]> = [];
    private absorb_data: Array<[number, Array<[number, number]>]> = [];

    constructor(
        private meter_heal_service: MeterHealService,
        private meter_absorb_service: MeterAbsorbService
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
    }

    private async merge_data(): Promise<void> {
        this.heal_data = [];
        this.absorb_data = [];
        this.subscription?.unsubscribe();
        this.subscription = this.meter_absorb_service.get_data(this.current_mode, this.abilities$, this.units$)
            .subscribe(data => {
                this.absorb_data = data;
                this.merge_meters();
            });
        this.subscription = this.meter_heal_service.get_data(HealMode.Effective, this.current_mode, this.abilities$, this.units$)
            .subscribe(data =>  {
                this.heal_data = data;
                this.merge_meters();
            });
    }

    private merge_meters(): void {
        const result = new Map();

        for (const data_set of [this.heal_data, this.absorb_data]) {
            for (const [subject_id, abilities] of data_set) {
                if (result.has(subject_id)) result.set(subject_id, [...result.get(subject_id), ...abilities]);
                else result.set(subject_id, abilities);
            }
        }

        this.data$.next([...result.entries()]);
    }
}
