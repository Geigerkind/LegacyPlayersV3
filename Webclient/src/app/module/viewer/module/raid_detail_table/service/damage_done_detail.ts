import {Injectable, OnDestroy} from "@angular/core";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {DetailRow} from "../domain_value/detail_row";
import {Event} from "../../../domain_value/event";
import {HitType} from "../../../domain_value/hit_type";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Damage} from "../../../domain_value/damage";
import {map, take} from "rxjs/operators";
import {SpellCast} from "../../../domain_value/spell_cast";
import {DelayedLabel} from "../../../../../stdlib/delayed_label";
import {SpellService} from "../../../service/spell";
import {Mitigation} from "../../../domain_value/mitigation";
import {SpellDamage} from "../../../domain_value/spell_damage";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL, CONST_UNKNOWN_LABEL} from "../../../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class DamageDoneDetailService implements OnDestroy {

    private subscription_changed: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private spell_damage: Array<Event> = [];
    private melee_damage: Map<number, Event> = new Map();

    constructor(
        private instanceDataService: InstanceDataService,
        private spellService: SpellService
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
        this.instanceDataService.aura_applications.pipe(take(1)).subscribe(aura_applications => {
            this.aura_applications = aura_applications;
            this.commit();
        });
        this.instanceDataService.spell_casts.pipe(take(1)).subscribe(spell_casts => {
            this.spell_casts = spell_casts;
            this.commit();
        });
        this.instanceDataService.spell_damage.pipe(take(1)).subscribe(spell_damage => {
            this.spell_damage = spell_damage;
            this.commit();
        });
        this.instanceDataService.melee_damage.pipe(take(1)).subscribe(melee_damage => {
            this.melee_damage = melee_damage;
            this.commit();
        });

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.AuraApplication].includes(changed))
                this.instanceDataService.aura_applications.pipe(take(1)).subscribe(aura_applications => {
                    this.aura_applications = aura_applications;
                    this.commit();
                });
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
        const abilities = new Map<number, DelayedLabel | string>();
        const ability_details = new Map<number, Map<HitType, DetailRow>>();
        if (this.melee_damage.size > 0) {
            abilities.set(CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL);
            const melee_details = new Map<HitType, DetailRow>();
            for (const event of [...this.melee_damage.values()]) {
                const damage = (event.event as any).MeleeDamage as Damage;
                this.fill_details(melee_details, damage);
            }
            ability_details.set(CONST_AUTO_ATTACK_ID, melee_details);
        }

        for (const event of this.spell_damage) {
            const spell_cause_id = ((event.event as any).SpellDamage as SpellDamage).spell_cause_id;
            const spell_cause_event = this.spell_casts.has(spell_cause_id) ? this.spell_casts.get(spell_cause_id) : this.aura_applications.get(spell_cause_id);
            if (!spell_cause_event)
                return;
            const hit_type = !!(spell_cause_event.event as any).SpellCast ? (spell_cause_event.event as any).SpellCast.hit_type : HitType.Hit;
            const spell_id = !!(spell_cause_event.event as any).SpellCast ? (spell_cause_event.event as any).SpellCast.spell_id : (spell_cause_event.event as any).AuraApplication.spell_id;
            const damage = (event.event as any).SpellDamage.damage as Damage;
            if (!ability_details.has(spell_id)) {
                abilities.set(spell_id, (new DelayedLabel(this.spellService.get_localized_basic_spell(spell_id)
                    .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
                ability_details.set(spell_id, new Map());
            }
            const details_map = ability_details.get(spell_id);
            this.fill_details(details_map, {
                damage: damage.damage,
                hit_type,
                mitigation: damage.mitigation,
                victim: undefined
            });
        }

        for (const event of [...this.spell_casts.values()]) {
            const spell_cast = (event.event as any).SpellCast as SpellCast;
            if ([HitType.Hit, HitType.Crit].includes(spell_cast.hit_type))
                continue;
            if (!ability_details.has(spell_cast.spell_id)) {
                abilities.set(spell_cast.spell_id, (new DelayedLabel(this.spellService.get_localized_basic_spell(spell_cast.spell_id)
                    .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
                ability_details.set(spell_cast.spell_id, new Map());
            }
            const details_map = ability_details.get(spell_cast.spell_id);
            this.fill_details(details_map, {
                damage: 0,
                hit_type: spell_cast.hit_type,
                mitigation: [],
                victim: undefined
            });
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

    private fill_details(details_map: Map<HitType, DetailRow>, damage: Damage): void {
        if (details_map.has(damage.hit_type)) {
            const details = details_map.get(damage.hit_type);
            ++details.count;
            details.amount += damage.damage;
            details.min = Math.min(details.min, damage.damage);
            details.max = Math.max(details.max, damage.damage);
            details.absorb += this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Absorb);
            details.block += this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Block);
            details.glance_or_resist += this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Resist)
                + this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Glance);
        } else {
            details_map.set(damage.hit_type, {
                amount: damage.damage,
                amount_percent: 0,
                average: 0,
                count: 1,
                count_percent: 0,
                hit_type: damage.hit_type,
                max: damage.damage,
                min: damage.damage,
                glance_or_resist: this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Resist)
                    + this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Glance),
                block: this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Block),
                absorb: this.extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Absorb)
            });
        }
    }

    private extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
        for (const mitigation of mitigations) {
            if (extract_function(mitigation) !== undefined)
                return extract_function(mitigation) as number;
        }
        return 0;
    }
}
