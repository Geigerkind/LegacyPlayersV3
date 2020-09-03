import {Threat} from "../../../domain_value/event";
import {hit_mask_to_hit_type_array, HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {detail_row_post_processing, fill_details} from "./util";
import {School} from "../../../domain_value/school";

export function commit_threat_detail(threat: Array<Threat>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    for (const event of threat) {
        const spell_id = event[5];
        const hit_mask = hit_mask_to_hit_type_array(event[6]);
        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());

        const details_map = ability_details.get(spell_id);

        fill_details([[event[7], event[8], 0, 0, 0]], hit_mask, details_map);
    }

    return detail_row_post_processing(ability_details);
}
