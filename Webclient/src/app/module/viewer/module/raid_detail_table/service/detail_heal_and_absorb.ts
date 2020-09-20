import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {InstanceDataService} from "../../../service/instance_data";
import {SpellService} from "../../../service/spell";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import {ABSORBING_SPELL_IDS, get_absorb_data_points} from "../../../stdlib/absorb";
import {detail_row_post_processing, fill_details} from "../stdlib/util";
import {HealMode} from "../../../domain_value/heal_mode";
import {school_array_to_school_mask} from "../../../domain_value/school";

@Injectable({
    providedIn: "root",
})
export class DetailHealAndAbsorbService implements OnDestroy {

    private subscription: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private initialized: boolean = false;

    private current_mode: boolean = false;

    constructor(
        private instanceDataService: InstanceDataService,
        private spellService: SpellService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get_ability_and_details(mode: boolean): [Observable<Array<SelectOption>>, Observable<Array<[number, Array<[HitType, DetailRow]>]>>] {
        if (!this.initialized) {
            this.current_mode = mode;
            this.initialize();
        } else if (this.current_mode !== mode) {
            this.current_mode = mode;
            this.commit();
        }
        return [this.abilities$.asObservable(), this.ability_details$.asObservable()];
    }

    private initialize(): void {
        this.initialized = true;
        this.subscription = this.instanceDataService.knecht_updates.subscribe(async ([knecht_update, evt_types]) => {
            if (knecht_update.includes(KnechtUpdates.FilterChanged) || (knecht_update.includes(KnechtUpdates.NewData) && [6, 12, 13].some(evt => evt_types.includes(evt))))
                this.commit();
        });
        this.commit();
    }

    private async commit(): Promise<void> {
        const abilities = new Map<number, SelectOption>();
        const result = new Map();

        const absorbs = await get_absorb_data_points(this.current_mode, this.instanceDataService);
        const heal = await this.instanceDataService.knecht_heal.detail_heal(HealMode.Effective, this.current_mode);
        for (const [subject_id, [subject, points]] of absorbs) {
            for (const [absorbed_spell_id, timestamp, amount] of points) {
                if (!abilities.has(absorbed_spell_id))
                    abilities.set(absorbed_spell_id, { value: absorbed_spell_id, label_key: this.spellService.get_spell_name(absorbed_spell_id) });
                if (!result.has(absorbed_spell_id))
                    result.set(absorbed_spell_id, new Map());
                const details_map = result.get(absorbed_spell_id);
                fill_details([[amount, school_array_to_school_mask(ABSORBING_SPELL_IDS.get(absorbed_spell_id)[1]), 0, 0, 0]], [HitType.Hit], details_map);
            }
        }

        for (const [ability_id, detail_rows] of heal) {
            if (!abilities.has(ability_id))
                abilities.set(ability_id, {value: ability_id, label_key: this.spellService.get_spell_name(ability_id)});
        }

        // @ts-ignore
        this.abilities$.next([...abilities.values()]);
        this.ability_details$.next([...detail_row_post_processing(result), ...heal]);
    }
}
