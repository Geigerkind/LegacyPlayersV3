import {Event} from "../../../domain_value/event";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_threat
} from "../../../extractor/events";
import {detail_row_post_processing} from "./util";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";

function commit_threat_detail(threat: Array<Event>, event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    return [];
    /*
    const ability_details = new Map<number, Map<HitType, DetailRow>>();

    for (const event of threat) {
        const threat_event = get_threat(event);
        const [indicator, spell_cause_event] = get_spell_cause(threat_event.cause_event_id, event_map);

        let hit_type;
        let spell_id;
        if (!spell_cause_event) {
            const melee_damage_event = event_map.get(threat_event.cause_event_id);
            if (!melee_damage_event)
                return;

            hit_type = get_melee_damage(melee_damage_event).hit_mask[0];
            spell_id = 0;
        } else {
            hit_type = indicator ? get_spell_cast(spell_cause_event).hit_mask[0] : HitType.Hit;
            spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
        }

        const threat_amount = threat_event.threat.amount;
        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());

        const details_map = ability_details.get(spell_id);

        if (details_map.has(hit_type)) {
            const details = details_map.get(hit_type);
            ++details.count;
            details.amount += threat_amount;
            details.min = Math.min(details.min, threat_amount);
            details.max = Math.max(details.max, threat_amount);
        } else {
            details_map.set(hit_type, {
                amount: threat_amount,
                amount_percent: 0,
                average: 0,
                count: 1,
                count_percent: 0,
                hit_type,
                max: threat_amount,
                min: threat_amount,
                glance_or_resist: 0,
                block: 0,
                absorb: 0
            });
        }
    }

    detail_row_post_processing(ability_details);
    return create_array_from_nested_map(ability_details);

     */
}

export {commit_threat_detail};
