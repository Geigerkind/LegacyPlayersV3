import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {Event} from "../../../domain_value/event";
import {get_aura_application, get_heal, get_spell_cast, get_spell_cause} from "../../../extractor/events";
import {HealMode} from "../../../domain_value/heal_mode";
import {detail_row_post_processing, fill_details} from "./util";
import {School} from "../../../domain_value/school";

function commit_heal_detail(current_mode: HealMode, heal: Array<Event>, event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    for (const event of heal) {
        const heal_event = get_heal(event);
        const [indicator, spell_cause_event] = get_spell_cause(heal_event.spell_cause_id, event_map);
        if (!spell_cause_event)
            return;
        const hit_mask = indicator ? get_spell_cast(spell_cause_event).hit_mask : heal_event.heal.hit_mask;
        const spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;

        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());
        const details_map = ability_details.get(spell_id);
        let healing;
        if (current_mode === HealMode.Total) healing = heal_event.heal.total;
        else if (current_mode === HealMode.Effective) healing = heal_event.heal.effective;
        else healing = heal_event.heal.total - heal_event.heal.effective;
        if (current_mode !== HealMode.Overheal || healing > 0)
            fill_details([{
                amount: healing,
                school_mask: [School.Holy], // TODO
                mitigation: heal_event.heal.mitigation
            }], hit_mask, details_map);
    }

    return detail_row_post_processing(ability_details);
}

export {commit_heal_detail};
