import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";
import {Damage} from "../../../domain_value/damage";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Mitigation} from "../../../domain_value/mitigation";
import {Event} from "../../../domain_value/event";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_spell_damage
} from "../../../extractor/events";
import {detail_row_post_processing} from "./util";

function commit_damage_detail(spell_damage: Array<Event>, melee_damage: Array<Event>, spell_casts: Array<Event>,
                              event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, DetailRow>>();

    if (melee_damage.length > 0) {
        const melee_details = new Map<HitType, DetailRow>();
        for (const event of melee_damage)
            fill_details(melee_details, get_melee_damage(event));
        ability_details.set(CONST_AUTO_ATTACK_ID, melee_details);
    }

    if (spell_damage.length > 0) {
        for (const event of spell_damage) {
            const spell_damage_event = get_spell_damage(event);
            const [indicator, spell_cause_event] = get_spell_cause(spell_damage_event.spell_cause_id, event_map);
            if (!spell_cause_event)
                return;
            const hit_type = indicator ? get_spell_cast(spell_cause_event).hit_type : HitType.Hit;
            const spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
            const damage = spell_damage_event.damage as Damage;
            if (!ability_details.has(spell_id))
                ability_details.set(spell_id, new Map());
            const details_map = ability_details.get(spell_id);
            fill_details(details_map, {
                damage: damage.damage,
                hit_type,
                mitigation: damage.mitigation,
                victim: undefined
            });
        }
    }

    if (spell_casts.length > 0) {
        for (const event of spell_casts) {
            const spell_cast = get_spell_cast(event);
            if ([HitType.Hit, HitType.Crit].includes(spell_cast.hit_type))
                continue;
            if (!ability_details.has(spell_cast.spell_id))
                ability_details.set(spell_cast.spell_id, new Map());
            const details_map = ability_details.get(spell_cast.spell_id);
            fill_details(details_map, {
                damage: 0,
                hit_type: spell_cast.hit_type,
                mitigation: [],
                victim: undefined
            });
        }
    }

    detail_row_post_processing(ability_details);
    return create_array_from_nested_map(ability_details);
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
