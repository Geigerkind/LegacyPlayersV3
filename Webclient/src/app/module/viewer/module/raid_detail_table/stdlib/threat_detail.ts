import {Event} from "../../../domain_value/event";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_threat
} from "../../../extractor/events";
import {detail_row_post_processing, fill_details} from "./util";
import {School} from "../../../domain_value/school";

function commit_threat_detail(threat: Array<Event>, event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    for (const event of threat) {
        const threat_event = get_threat(event);
        const [indicator, spell_cause_event] = get_spell_cause(threat_event.cause_event_id, event_map);

        let hit_mask;
        let spell_id;
        if (!spell_cause_event) {
            const melee_damage_event = event_map.get(threat_event.cause_event_id);
            if (!melee_damage_event)
                return;

            hit_mask = get_melee_damage(melee_damage_event).hit_mask;
            spell_id = 0;
        } else {
            hit_mask = indicator ? get_spell_cast(spell_cause_event).hit_mask : [HitType.Hit];
            spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
        }

        const threat_amount = threat_event.threat.amount;
        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());

        const details_map = ability_details.get(spell_id);

        fill_details([{
            amount: threat_amount,
            school_mask: [School.Holy], // TODO
            mitigation: []
        }], hit_mask, details_map);
    }

    return detail_row_post_processing(ability_details);
}

export {commit_threat_detail};
