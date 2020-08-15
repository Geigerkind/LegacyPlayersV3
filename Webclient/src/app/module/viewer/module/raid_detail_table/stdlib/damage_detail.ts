import {DelayedLabel} from "../../../../../stdlib/delayed_label";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL, CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {Damage} from "../../../domain_value/damage";
import {map} from "rxjs/operators";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Mitigation} from "../../../domain_value/mitigation";
import {Event} from "../../../domain_value/event";
import {SpellService} from "../../../service/spell";
import {BehaviorSubject} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {get_aura_application, get_melee_damage, get_spell_cast, get_spell_damage} from "../../../extractor/events";
import {detail_row_post_processing} from "./util";

function commit_damage_detail(spellService: SpellService, ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>>,
                              abilities$: BehaviorSubject<Array<SelectOption>>, spell_damage: Array<Event>,
                              melee_damage: Map<number, Event>, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): void {
    const abilities = new Map<number, DelayedLabel | string>();
    const ability_details = new Map<number, Map<HitType, DetailRow>>();

    if (melee_damage.size > 0) {
        abilities.set(CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL);
        const melee_details = new Map<HitType, DetailRow>();
        // @ts-ignore
        for (const event of [...melee_damage.values()])
            fill_details(melee_details, get_melee_damage(event));
        ability_details.set(CONST_AUTO_ATTACK_ID, melee_details);
    }

    for (const event of spell_damage) {
        const spell_damage_event = get_spell_damage(event);
        const spell_cause_id = spell_damage_event.spell_cause_id;
        const spell_cause_event = spell_casts.has(spell_cause_id) ? spell_casts.get(spell_cause_id) : aura_applications.get(spell_cause_id);
        if (!spell_cause_event)
            return;
        const spell_cast_event = get_spell_cast(spell_cause_event);
        const hit_type = !!spell_cast_event ? spell_cast_event.hit_type : HitType.Hit;
        const spell_id = !!spell_cast_event ? spell_cast_event.spell_id : get_aura_application(spell_cause_event).spell_id;
        const damage = spell_damage_event.damage as Damage;
        if (!ability_details.has(spell_id)) {
            abilities.set(spell_id, (new DelayedLabel(spellService.get_localized_basic_spell(spell_id)
                .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
            ability_details.set(spell_id, new Map());
        }
        const details_map = ability_details.get(spell_id);
        fill_details(details_map, {
            damage: damage.damage,
            hit_type,
            mitigation: damage.mitigation,
            victim: undefined
        });
    }

    // @ts-ignore
    for (const event of [...spell_casts.values()]) {
        const spell_cast = get_spell_cast(event);
        if ([HitType.Hit, HitType.Crit].includes(spell_cast.hit_type))
            continue;
        if (!ability_details.has(spell_cast.spell_id)) {
            abilities.set(spell_cast.spell_id, (new DelayedLabel(spellService.get_localized_basic_spell(spell_cast.spell_id)
                .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
            ability_details.set(spell_cast.spell_id, new Map());
        }
        const details_map = ability_details.get(spell_cast.spell_id);
        fill_details(details_map, {
            damage: 0,
            hit_type: spell_cast.hit_type,
            mitigation: [],
            victim: undefined
        });
    }

    detail_row_post_processing(ability_details);

    // @ts-ignore
    abilities$.next([...abilities.entries()].map(([value, label_key]) => {
        return {value, label_key};
    }));
    ability_details$.next(create_array_from_nested_map(ability_details));
}

function fill_details(details_map: Map<HitType, DetailRow>, damage: Damage): void {
    if (details_map.has(damage.hit_type)) {
        const details = details_map.get(damage.hit_type);
        ++details.count;
        details.amount += damage.damage;
        details.min = Math.min(details.min, damage.damage);
        details.max = Math.max(details.max, damage.damage);
        details.absorb += extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Absorb);
        details.block += extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Block);
        details.glance_or_resist += extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Resist)
            + extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Glance);
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
            glance_or_resist: extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Resist)
                + extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Glance),
            block: extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Block),
            absorb: extract_mitigation_amount(damage.mitigation, (mitigation) => mitigation.Absorb)
        });
    }
}

function extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
    for (const mitigation of mitigations) {
        if (extract_function(mitigation) !== undefined)
            return extract_function(mitigation) as number;
    }
    return 0;
}

export {commit_damage_detail};
