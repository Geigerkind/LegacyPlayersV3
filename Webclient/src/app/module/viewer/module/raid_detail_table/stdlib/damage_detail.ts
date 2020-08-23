import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_ID_MH, CONST_AUTO_ATTACK_ID_OH} from "../../../constant/viewer";
import {Event} from "../../../domain_value/event";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_spell_damage
} from "../../../extractor/events";
import {School} from "../../../domain_value/school";
import {detail_row_post_processing, fill_details} from "./util";

function commit_damage_detail(spell_damage: Array<Event>, melee_damage: Array<Event>, spell_casts: Array<Event>,
                              event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    if (melee_damage.length > 0) {
        const melee_details = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_mh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_oh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        for (const event of melee_damage) {
            const target_event = get_melee_damage(event);
            fill_details(target_event.components, target_event.hit_mask, melee_details);
            if (target_event.hit_mask.includes(HitType.OffHand))
                fill_details(target_event.components, target_event.hit_mask, melee_details_oh);
            else fill_details(target_event.components, target_event.hit_mask, melee_details_mh);
        }
        ability_details.set(CONST_AUTO_ATTACK_ID, melee_details);
        if (melee_details_oh.size > 0 && melee_details_mh.size > 0) {
            ability_details.set(CONST_AUTO_ATTACK_ID_OH, melee_details_oh);
            ability_details.set(CONST_AUTO_ATTACK_ID_MH, melee_details_mh);
        }
    }
    if (spell_damage.length > 0) {
        for (const event of spell_damage) {
            const spell_damage_event = get_spell_damage(event);
            const [indicator, spell_cause_event] = get_spell_cause(spell_damage_event.spell_cause_id, event_map);
            if (!spell_cause_event)
                return;
            const hit_mask = indicator ? get_spell_cast(spell_cause_event).hit_mask : spell_damage_event.damage.hit_mask;
            const spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
            if (!ability_details.has(spell_id))
                ability_details.set(spell_id, new Map());
            const details_map = ability_details.get(spell_id);
            fill_details(spell_damage_event.damage.components, hit_mask, details_map);
        }
    }

    if (spell_casts.length > 0) {
        for (const event of spell_casts) {
            const spell_cast = get_spell_cast(event);
            if (spell_cast.hit_mask.includes(HitType.Crit) || spell_cast.hit_mask.includes(HitType.Hit) || spell_cast.hit_mask.includes(HitType.Crushing))
                continue;
            if (!ability_details.has(spell_cast.spell_id))
                ability_details.set(spell_cast.spell_id, new Map());
            const details_map = ability_details.get(spell_cast.spell_id);
            fill_details([], spell_cast.hit_mask, details_map);
        }
    }

    return detail_row_post_processing(ability_details);
}

export {commit_damage_detail};
