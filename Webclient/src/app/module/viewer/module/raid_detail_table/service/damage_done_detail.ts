import {Injectable, OnDestroy} from "@angular/core";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {DetailRow} from "../domain_value/detail_row";
import {Event} from "../../../domain_value/event";
import {HitType} from "../../../domain_value/hit_type";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Damage} from "../../../domain_value/damage";
import {take} from "rxjs/operators";

@Injectable({
    providedIn: "root",
})
export class DamageDoneDetailService implements OnDestroy {

    private subscription_changed: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private initialized: boolean = false;

    private spell_casts: Array<Event> = [];
    private spell_damage: Array<Event> = [];
    private melee_damage: Array<Event> = [];

    constructor(
        private instanceDataService: InstanceDataService,
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get ability_and_details(): [Observable<Array<SelectOption>>, Observable<Array<[number, Array<[HitType, DetailRow]>]>>] {
        if (!this.initialized)
            this.initialize();
        return [this.abilities$.asObservable(), this.ability_details$.asObservable()];
    }

    private initialize(): void {
        this.initialized = true;

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellCast].includes(changed))
                this.instanceDataService.spell_casts.pipe(take(1)).subscribe(spell_casts => {
                    this.spell_casts = spell_casts;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellDamage].includes(changed))
                this.instanceDataService.spell_damage.pipe(take(1)).subscribe(spell_damage => {
                    this.spell_damage = spell_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.MeleeDamage].includes(changed))
                this.instanceDataService.melee_damage.pipe(take(1)).subscribe(melee_damage => {
                    this.melee_damage = melee_damage;
                    this.commit();
                });
        });
    }

    private commit(): void {
        const abilities = new Map<number, string>();
        const ability_details = new Map<number, Map<HitType, DetailRow>>();
        if (this.melee_damage.length > 0) {
            abilities.set(0, "Auto Attack");
            const melee_details = new Map<HitType, DetailRow>();
            for (const event of this.melee_damage) {
                const damage = (event.event as any).MeleeDamage as Damage;
                if (melee_details.has(damage.hit_type)) {
                    const details = melee_details.get(damage.hit_type);
                    ++details.count;
                    details.amount += damage.damage;
                    details.min = Math.min(details.min, damage.damage);
                    details.max = Math.max(details.max, damage.damage);
                } else {
                    melee_details.set(damage.hit_type, {
                        amount: damage.damage,
                        amount_percent: 0,
                        average: 0,
                        count: 1,
                        count_percent: 0,
                        hit_type: damage.hit_type,
                        max: damage.damage,
                        min: damage.damage
                    });
                }
            }
            ability_details.set(0, melee_details);
        }

        // Post processing
        for (const [ability, hit_types] of ability_details) {
            let total_count = 0;
            let total_amount = 0;
            for (const [hit_type, details] of hit_types) {
                total_count += details.count;
                total_amount += details.amount;
            }

            for (const [hit_type, details] of hit_types) {
                details.amount_percent = 100 * (details.amount / total_amount);
                details.count_percent = 100 * (details.count / total_count);
                details.average = details.amount / details.count;
            }
        }

        this.abilities$.next([...abilities.entries()].map(([value, label_key]) => {
            return {value, label_key};
        }));
        this.ability_details$.next(create_array_from_nested_map(ability_details));
    }
}
