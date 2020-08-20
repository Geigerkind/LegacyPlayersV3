import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {InstanceDataService} from "../../../service/instance_data";
import {SpellService} from "../../../service/spell";
import {HealMode} from "../../../domain_value/heal_mode";

@Injectable({
    providedIn: "root",
})
export class DetailHealService implements OnDestroy {

    private subscription: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private initialized: boolean = false;

    private current_mode: boolean = false;
    private current_heal_mode: HealMode = HealMode.Total;

    constructor(
        private instanceDataService: InstanceDataService,
        private spellService: SpellService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get_ability_and_details(heal_mode: HealMode, mode: boolean): [Observable<Array<SelectOption>>, Observable<Array<[number, Array<[HitType, DetailRow]>]>>] {
        if (!this.initialized) {
            this.current_mode = mode;
            this.initialize();
        } else if (this.current_mode !== mode || this.current_heal_mode !== heal_mode) {
            this.current_mode = mode;
            this.current_heal_mode = heal_mode;
            this.commit();
        }
        return [this.abilities$.asObservable(), this.ability_details$.asObservable()];
    }

    private initialize(): void {
        this.initialized = true;
        this.subscription = this.instanceDataService.knecht_updates.subscribe(async () => this.commit());
        this.commit();
    }

    private async commit(): Promise<void> {
        const abilities = new Map<number, SelectOption>();

        const result1 = await this.instanceDataService.knecht_spell.detail_heal(this.current_heal_mode, this.current_mode);
        for (const [ability_id, detail_rows] of result1) {
            if (!abilities.has(ability_id))
                abilities.set(ability_id, {value: ability_id, label_key: this.spellService.get_spell_name(ability_id)});
        }

        // @ts-ignore
        this.abilities$.next([...abilities.values()]);
        this.ability_details$.next(result1);
    }
}
