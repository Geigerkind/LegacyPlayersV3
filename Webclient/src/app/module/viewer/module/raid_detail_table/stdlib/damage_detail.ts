import {hit_mask_to_hit_type_array, HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_ID_MH, CONST_AUTO_ATTACK_ID_OH} from "../../../constant/viewer";
import {MeleeDamage, SpellDamage} from "../../../domain_value/event";
import {School} from "../../../domain_value/school";
import {detail_row_post_processing, fill_details} from "./util";

export function commit_damage_detail(spell_damage: Array<SpellDamage>, melee_damage: Array<MeleeDamage>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    if (melee_damage.length > 0) {
        const melee_details = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_mh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_oh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        for (const event of melee_damage) {
            const hit_mask = hit_mask_to_hit_type_array(event[4]);
            fill_details(event[5], hit_mask, melee_details);
            if (hit_mask.includes(HitType.OffHand))
                fill_details(event[5], hit_mask, melee_details_oh);
            else fill_details(event[5], hit_mask, melee_details_mh);
        }
        ability_details.set(CONST_AUTO_ATTACK_ID, melee_details);
        if (melee_details_oh.size > 0 && melee_details_mh.size > 0) {
            ability_details.set(CONST_AUTO_ATTACK_ID_OH, melee_details_oh);
            ability_details.set(CONST_AUTO_ATTACK_ID_MH, melee_details_mh);
        }
    }
    if (spell_damage.length > 0) {
        for (const event of spell_damage) {
            const hit_mask = hit_mask_to_hit_type_array(event[6]);
            const spell_id = event[5];
            if (!ability_details.has(spell_id))
                ability_details.set(spell_id, new Map());
            const details_map = ability_details.get(spell_id);
            fill_details(event[7], hit_mask, details_map);
        }
    }

    return detail_row_post_processing(ability_details);
}
