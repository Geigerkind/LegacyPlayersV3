import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {DetailRow} from "../domain_value/detail_row";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {DamageDoneDetailService} from "./damage_done_detail";
import {HitType} from "../../../domain_value/hit_type";

@Injectable({
    providedIn: "root",
})
export class RaidDetailService implements OnDestroy {

    private subscription_ability: Subscription;
    private subscription_ability_details: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private current_selection: number = -1;

    constructor(
        private damageDoneDetailService: DamageDoneDetailService
    ) {}

    ngOnDestroy(): void {
    }

    get abilities(): Observable<Array<SelectOption>> {
        return this.abilities$.asObservable();
    }

    get ability_details(): Observable<Array<[number, Array<[HitType, DetailRow]>]>> {
        return this.ability_details$.asObservable();
    }

    select(selection: number): void {
        if (this.current_selection === selection)
            return;

        this.subscription_ability?.unsubscribe();
        this.subscription_ability_details?.unsubscribe();

        switch (selection) {
            case 1:
                const [abilities, ability_details] = this.damageDoneDetailService.ability_and_details;
                this.subscription_ability = abilities.subscribe(i_abilities => this.abilities$.next(i_abilities));
                this.subscription_ability_details = ability_details.subscribe(i_ability_details => this.ability_details$.next(i_ability_details));
                break;
        }

        this.current_selection = selection;
    }
}
