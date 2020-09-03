import {hit_mask_to_hit_type_array, HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {Heal} from "../../../domain_value/event";
import {HealMode} from "../../../domain_value/heal_mode";
import {detail_row_post_processing, fill_details} from "./util";
import {School} from "../../../domain_value/school";

export function commit_heal_detail(current_mode: HealMode, heal: Array<Heal>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    for (const event of heal) {
        const hit_mask = hit_mask_to_hit_type_array(event[6]);
        const spell_id = event[5];
        const school_mask = event[7];

        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());
        const details_map = ability_details.get(spell_id);
        let healing;
        if (current_mode === HealMode.Total) healing = event[8];
        else if (current_mode === HealMode.Effective) healing = event[9];
        else healing = event[8] - event[9];
        if (current_mode !== HealMode.Overheal || healing > 0)
            fill_details([[healing, school_mask, event[10], 0, 0]], hit_mask, details_map);
    }

    return detail_row_post_processing(ability_details);
}
