import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {Event} from "../../../domain_value/event";
import {get_aura_application, get_heal, get_spell_cast, get_spell_cause} from "../../../extractor/events";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Mitigation} from "../../../domain_value/mitigation";
import {HealMode} from "../../../domain_value/heal_mode";
import {detail_row_post_processing} from "./util";

function commit_heal_detail(current_mode: HealMode, heal: Array<Event>, event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, DetailRow>>();

    for (const event of heal) {
        const heal_event = get_heal(event);
        const [indicator, spell_cause_event] = get_spell_cause(heal_event.spell_cause_id, event_map);
        if (!spell_cause_event)
            return;
        const hit_type = indicator ? get_spell_cast(spell_cause_event).hit_type : HitType.Hit;
        const spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;

        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());
        const details_map = ability_details.get(spell_id);
        let healing;
        if (current_mode === HealMode.Total) healing = heal_event.heal.total;
        else if (current_mode === HealMode.Effective) healing = heal_event.heal.effective;
        else healing = heal_event.heal.total - heal_event.heal.effective;
        if (current_mode !== HealMode.Overheal || healing > 0) {
            if (details_map.has(hit_type)) {
                const details = details_map.get(hit_type);
                ++details.count;
                details.amount += healing;
                details.min = Math.min(details.min, healing);
                details.max = Math.max(details.max, healing);
                details.absorb += extract_mitigation_amount(heal_event.heal.mitigation, (mitigation) => mitigation.Absorb);
            } else {
                details_map.set(hit_type, {
                    amount: healing,
                    amount_percent: 0,
                    average: 0,
                    count: 1,
                    count_percent: 0,
                    hit_type,
                    max: healing,
                    min: healing,
                    glance_or_resist: 0,
                    block: 0,
                    absorb: extract_mitigation_amount(heal_event.heal.mitigation, (mitigation) => mitigation.Absorb)
                });
            }
        }
    }

    detail_row_post_processing(ability_details);
    return create_array_from_nested_map(ability_details);
}

function extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
    for (const mitigation of mitigations) {
        if (extract_function(mitigation) !== undefined)
            return extract_function(mitigation) as number;
    }
    return 0;
}

export {commit_heal_detail};
