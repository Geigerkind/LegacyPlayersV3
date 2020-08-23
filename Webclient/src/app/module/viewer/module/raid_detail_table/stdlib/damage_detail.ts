import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent, ResistSummary, ResistSummaryRow} from "../domain_value/detail_row";
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
import {Mitigation} from "../../../domain_value/mitigation";
import {Damage} from "../../../domain_value/damage";

const categories = [
    HitType.None,
    HitType.Hit,
    HitType.Crushing,
    HitType.Crit,
    HitType.Glancing,
    HitType.Parry,
    HitType.Dodge,
    HitType.Miss,
    HitType.Environment,
    HitType.Evade,
    HitType.Immune
];

function find_category(hit_mask: Array<HitType>): HitType {
    for (const hit_type of hit_mask) {
        if (categories.includes(hit_type))
            return hit_type;
    }
    return HitType.None;
}

function commit_damage_detail(spell_damage: Array<Event>, melee_damage: Array<Event>, spell_casts: Array<Event>,
                              event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    if (melee_damage.length > 0) {
        const melee_details = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_mh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        const melee_details_oh = new Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>();
        for (const event of melee_damage) {
            const target_event = get_melee_damage(event);
            fill_details(target_event, target_event.hit_mask, melee_details);
            if (target_event.hit_mask.includes(HitType.OffHand))
                fill_details(target_event, target_event.hit_mask, melee_details_oh);
            else fill_details(target_event, target_event.hit_mask, melee_details_mh);
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
            fill_details(spell_damage_event.damage, hit_mask, details_map);
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
            fill_details({damage_components: []} as Damage, spell_cast.hit_mask, details_map);
        }
    }

    // Post Processing
    const result = [];
    for (const [spell_id, hit_categories] of ability_details) {
        let total_count = 0;
        let total_amount = 0;
        for (const [hit_type, [content, components]] of hit_categories) {
            total_count += content.count;
            total_amount += content.amount;
        }

        const detail_rows = [];
        for (const [hit_type, [content, components]] of hit_categories) {
            const detail_row_components = [];
            for (const [school, [comp_content, resists]] of components) {
                // Sort resists into 25, 50, 75 category
                const avg_percent_0 = (comp_content.resist_summary.percent_0.amount / comp_content.resist_summary.percent_0.count) || 0;
                for (const resist of resists) {
                    const frac = (100 * resist) / avg_percent_0;
                    if (frac >= 62.5) update_resist_summary_row(comp_content.resist_summary.percent_25, 1, resist, resist, resist);
                    else if (frac >= 37.5) update_resist_summary_row(comp_content.resist_summary.percent_50, 1, resist, resist, resist);
                    else update_resist_summary_row(comp_content.resist_summary.percent_75, 1, resist, resist, resist);
                }

                // Calculate resist summary average etc.
                post_process_resist_summary(comp_content.resist_summary);

                // Update parent resist summary
                update_resist_summary_row(content.resist_summary.percent_25, comp_content.resist_summary.percent_25.count, comp_content.resist_summary.percent_25.amount,
                    comp_content.resist_summary.percent_25.min, comp_content.resist_summary.percent_25.max);
                update_resist_summary_row(content.resist_summary.percent_50, comp_content.resist_summary.percent_50.count, comp_content.resist_summary.percent_50.amount,
                    comp_content.resist_summary.percent_50.min, comp_content.resist_summary.percent_50.max);
                update_resist_summary_row(content.resist_summary.percent_75, comp_content.resist_summary.percent_75.count, comp_content.resist_summary.percent_75.amount,
                    comp_content.resist_summary.percent_75.min, comp_content.resist_summary.percent_75.max);

                // Set comp content avg, and percents
                comp_content.average = comp_content.amount / comp_content.count;
                comp_content.amount_percent = (100 * comp_content.amount) / content.amount;
                comp_content.count_percent = (100 * comp_content.count) / content.count;

                detail_row_components.push({
                    school,
                    content: comp_content
                });
            }

            // Calculate resist summary average etc.
            post_process_resist_summary(content.resist_summary);

            // Set content avg, percent
            content.average = content.amount / content.count;
            content.amount_percent = (100 * content.amount) / total_amount;
            content.count_percent = (100 * content.count) / total_count;

            detail_rows.push([hit_type, {
                hit_type,
                content,
                components: detail_row_components
            }]);
        }

        result.push([spell_id, detail_rows]);
    }

    return result;
}

function fill_details(target_event: Damage, hit_mask: Array<HitType>, details: Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>): void {
    const category = find_category(hit_mask);

    if (!details.has(category)) {
        const i_components = new Map<School, [DetailRowContent, Array<number>]>();
        const row_summary: [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>] = [create_detail_row_content(), i_components];
        details.set(category, row_summary);
    }

    const [detail_row_content, components] = details.get(category);
    let total_amount = 0;
    let total_resist = 0;
    let total_block = 0;
    let total_absorb = 0;
    let total_percent_0 = 0;
    let total_percent_0_min = detail_row_content.resist_summary.percent_0.min;
    let total_percent_0_max = detail_row_content.resist_summary.percent_0.max;
    let total_percent_100 = 0;
    let total_percent_100_min = detail_row_content.resist_summary.percent_100.min;
    let total_percent_100_max = detail_row_content.resist_summary.percent_100.max;
    for (const component of target_event.damage_components) {
        if (!components.has(component.school_mask[0])) {
            components.set(component.school_mask[0], [create_detail_row_content(), []]);
        }
        const [comp_content, resists] = components.get(component.school_mask[0]);
        const resist = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Resist);
        const block = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Block);
        const absorb = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Absorb);
        total_amount += component.damage;
        total_resist += resist;
        total_block += block;
        total_absorb += absorb;
        const percent_0 = hit_mask.includes(HitType.PartialResist) || target_event.hit_mask.includes(HitType.FullResist) ? 0 : component.damage;
        total_percent_0 += percent_0;
        if (target_event.hit_mask.includes(HitType.PartialResist))
            resists.push(resist);
        const percent_100 = target_event.hit_mask.includes(HitType.FullResist) ? resist : 0;
        total_percent_100 += percent_100;

        if (percent_0 > 0) {
            total_percent_0_min = Math.min(total_percent_0_min, percent_0);
            total_percent_0_max = Math.min(total_percent_0_max, percent_0);
        }
        if (percent_100 > 0) {
            total_percent_100_min = Math.min(total_percent_100_min, percent_100);
            total_percent_100_max = Math.min(total_percent_100_max, percent_100);
        }

        update_detail_row_content(comp_content, component.damage, resist, block, absorb, percent_0, percent_0, percent_0, percent_100, percent_100, percent_100);
    }

    update_detail_row_content(detail_row_content, total_amount, total_resist, total_block, total_absorb, total_percent_0,
        total_percent_0_min, total_percent_0_max, total_percent_100, total_percent_100_min, total_percent_100_max);
}

function post_process_resist_summary(resist_summary: ResistSummary): void {
    const total_count = resist_summary.percent_0.count + resist_summary.percent_25.count + resist_summary.percent_50.count
        + resist_summary.percent_75.count + resist_summary.percent_100.count;
    const total_amount = resist_summary.percent_0.amount + resist_summary.percent_25.amount + resist_summary.percent_50.amount
        + resist_summary.percent_75.amount + resist_summary.percent_100.amount;

    resist_summary.percent_0.average = resist_summary.percent_0.count === 0 ? 0 : resist_summary.percent_0.amount / resist_summary.percent_0.count;
    resist_summary.percent_25.average = resist_summary.percent_25.count === 0 ? 0 : resist_summary.percent_25.amount / resist_summary.percent_25.count;
    resist_summary.percent_50.average = resist_summary.percent_50.count === 0 ? 0 : resist_summary.percent_50.amount / resist_summary.percent_50.count;
    resist_summary.percent_75.average = resist_summary.percent_75.count === 0 ? 0 : resist_summary.percent_75.amount / resist_summary.percent_75.count;
    resist_summary.percent_100.average = resist_summary.percent_100.count === 0 ? 0 : resist_summary.percent_100.amount / resist_summary.percent_100.count;

    resist_summary.percent_0.count_percent = total_count === 0 ? 0 : (100 * resist_summary.percent_0.count) / total_count;
    resist_summary.percent_25.count_percent = total_count === 0 ? 0 : (100 * resist_summary.percent_25.count) / total_count;
    resist_summary.percent_50.count_percent = total_count === 0 ? 0 : (100 * resist_summary.percent_50.count) / total_count;
    resist_summary.percent_75.count_percent = total_count === 0 ? 0 : (100 * resist_summary.percent_75.count) / total_count;
    resist_summary.percent_100.count_percent = total_count === 0 ? 0 : (100 * resist_summary.percent_100.count) / total_count;

    resist_summary.percent_0.amount_percent = total_amount === 0 ? 0 : (100 * resist_summary.percent_0.amount) / total_amount;
    resist_summary.percent_25.amount_percent = total_amount === 0 ? 0 : (100 * resist_summary.percent_25.amount) / total_amount;
    resist_summary.percent_50.amount_percent = total_amount === 0 ? 0 : (100 * resist_summary.percent_50.amount) / total_amount;
    resist_summary.percent_75.amount_percent = total_amount === 0 ? 0 : (100 * resist_summary.percent_75.amount) / total_amount;
    resist_summary.percent_100.amount_percent = total_amount === 0 ? 0 : (100 * resist_summary.percent_100.amount) / total_amount;

    if (resist_summary.percent_0.count === 0) resist_summary.percent_0.min = 0;
    if (resist_summary.percent_25.count === 0) resist_summary.percent_25.min = 0;
    if (resist_summary.percent_50.count === 0) resist_summary.percent_50.min = 0;
    if (resist_summary.percent_75.count === 0) resist_summary.percent_75.min = 0;
    if (resist_summary.percent_100.count === 0) resist_summary.percent_100.min = 0;
}

function update_resist_summary_row(resist_summary_row: ResistSummaryRow, count: number, amount: number, min: number, max: number): void {
    resist_summary_row.count += count;
    resist_summary_row.amount += amount;
    resist_summary_row.min = Math.min(min, resist_summary_row.min);
    resist_summary_row.max = Math.max(max, resist_summary_row.max);
}

function update_detail_row_content(comp_content: DetailRowContent, amount: number, resist: number, block: number, absorb: number, percent_0: number, percent_0_min: number,
                                   percent_0_max: number, percent_100: number, percent_100_min: number, percent_100_max: number): void {
    ++comp_content.count;
    comp_content.amount += amount;
    comp_content.resist += resist;
    comp_content.block += block;
    comp_content.absorb += absorb;
    comp_content.min = Math.min(amount, comp_content.min);
    comp_content.max = Math.max(amount, comp_content.max);
    if (percent_0 > 0) {
        ++comp_content.resist_summary.percent_0.count;
        comp_content.resist_summary.percent_0.amount += percent_0;
        comp_content.resist_summary.percent_0.min = Math.min(percent_0_min, comp_content.resist_summary.percent_0.min);
        comp_content.resist_summary.percent_0.max = Math.max(percent_0_max, comp_content.resist_summary.percent_0.max);
    }
    if (percent_100) {
        ++comp_content.resist_summary.percent_100.count;
        comp_content.resist_summary.percent_100.amount += percent_100;
        comp_content.resist_summary.percent_100.min = Math.min(percent_100_min, comp_content.resist_summary.percent_100.min);
        comp_content.resist_summary.percent_100.max = Math.max(percent_100_max, comp_content.resist_summary.percent_100.max);
    }
}

function create_resist_summary_row(): ResistSummaryRow {
    return {
        amount: 0,
        amount_percent: 0,
        count: 0,
        count_percent: 0,
        min: Number.MAX_VALUE,
        max: 0,
        average: 0
    };
}

function create_detail_row_content(): DetailRowContent {
    return {
        amount: 0,
        amount_percent: 0,
        average: 0,
        count: 0,
        count_percent: 0,
        max: 0,
        min: Number.MAX_VALUE,
        resist: 0,
        block: 0,
        absorb: 0,
        resist_summary: {
            percent_0: create_resist_summary_row(),
            percent_25: create_resist_summary_row(),
            percent_50: create_resist_summary_row(),
            percent_75: create_resist_summary_row(),
            percent_100: create_resist_summary_row(),
        } as ResistSummary
    };
}

function extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
    for (const mitigation of mitigations) {
        if (extract_function(mitigation) !== undefined)
            return extract_function(mitigation) as number;
    }
    return 0;
}

export {commit_damage_detail};
