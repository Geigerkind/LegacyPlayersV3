import {Mitigation} from "../../../domain_value/mitigation";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent, ResistSummary, ResistSummaryRow} from "../domain_value/detail_row";
import {School} from "../../../domain_value/school";
import {SpellComponent} from "../../../domain_value/spell_component";

export function detail_row_post_processing(ability_details: any): Array<[number, Array<[HitType, DetailRow]>]> {
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

export function extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
    for (const mitigation of mitigations) {
        if (extract_function(mitigation) !== undefined)
            return extract_function(mitigation) as number;
    }
    return 0;
}

export const DETAIL_CATEGORIES = [
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

export function find_category(hit_mask: Array<HitType>): HitType {
    for (const hit_type of hit_mask) {
        if (DETAIL_CATEGORIES.includes(hit_type))
            return hit_type;
    }
    return HitType.None;
}

export function fill_details(spell_components: Array<SpellComponent>, hit_mask: Array<HitType>, details: Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>): void {
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
    for (const component of spell_components) {
        if (!components.has(component.school_mask[0])) {
            components.set(component.school_mask[0], [create_detail_row_content(), []]);
        }
        const [comp_content, resists] = components.get(component.school_mask[0]);
        const resist = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Resist);
        const block = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Block);
        const absorb = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Absorb);
        total_amount += component.amount;
        total_resist += resist;
        total_block += block;
        total_absorb += absorb;
        const percent_0 = hit_mask.includes(HitType.PartialResist) || hit_mask.includes(HitType.FullResist) ? 0 : component.amount;
        total_percent_0 += percent_0;
        if (hit_mask.includes(HitType.PartialResist))
            resists.push(resist);
        const percent_100 = hit_mask.includes(HitType.FullResist) ? resist : 0;
        total_percent_100 += percent_100;

        if (percent_0 > 0) {
            total_percent_0_min = Math.min(total_percent_0_min, percent_0);
            total_percent_0_max = Math.min(total_percent_0_max, percent_0);
        }
        if (percent_100 > 0) {
            total_percent_100_min = Math.min(total_percent_100_min, percent_100);
            total_percent_100_max = Math.min(total_percent_100_max, percent_100);
        }

        update_detail_row_content(comp_content, component.amount, resist, block, absorb, percent_0, percent_0, percent_0, percent_100, percent_100, percent_100);
    }

    update_detail_row_content(detail_row_content, total_amount, total_resist, total_block, total_absorb, total_percent_0,
        total_percent_0_min, total_percent_0_max, total_percent_100, total_percent_100_min, total_percent_100_max);
}

export function post_process_resist_summary(resist_summary: ResistSummary): void {
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

export function update_resist_summary_row(resist_summary_row: ResistSummaryRow, count: number, amount: number, min: number, max: number): void {
    resist_summary_row.count += count;
    resist_summary_row.amount += amount;
    resist_summary_row.min = Math.min(min, resist_summary_row.min);
    resist_summary_row.max = Math.max(max, resist_summary_row.max);
}

export function update_detail_row_content(comp_content: DetailRowContent, amount: number, resist: number, block: number, absorb: number, percent_0: number, percent_0_min: number,
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

export function create_resist_summary_row(): ResistSummaryRow {
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

export function create_detail_row_content(): DetailRowContent {
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
