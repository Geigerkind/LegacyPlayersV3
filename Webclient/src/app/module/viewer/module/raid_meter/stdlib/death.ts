import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id, get_unit_owner, Unit} from "../../../domain_value/unit";
import {Event, MeleeDamage, SpellDamage} from "../../../domain_value/event";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";
import {se_death} from "../../../extractor/sources";
import {get_spell_components_total_amount} from "../../../domain_value/damage";
import {te_death} from "../../../extractor/targets";

function commit_death(deaths: Array<Event>, melee_damage: Array<MeleeDamage>,
                      spell_damage: Array<SpellDamage>, melee_unit_extraction: (Event) => Unit, spell_unit_extraction: (Event) => Unit,
                      death_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<DeathOverviewRow>]]> {
    const newData = new Map<number, [Unit, Array<DeathOverviewRow>]>();

    const grouping = group_by(deaths, (event) => get_unit_id(death_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        for (const event of grouping[unit_id]) {
            // For each death find the ability that executed this death
            let spell_id;
            let found_spell_min_ts = Number.MAX_VALUE;
            let amount = 0;

            const event_subject_id = get_unit_id(se_death(event));
            for (const md_event of melee_damage) {
                const difference = Math.abs(md_event[1] - event[1]);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(melee_unit_extraction(md_event)) === event_subject_id && difference < found_spell_min_ts) {
                    const kb_amount = get_spell_components_total_amount(md_event[5]);
                    if (kb_amount > 0) {
                        spell_id = 0;
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }

            for (const sd_event of spell_damage) {
                const difference = Math.abs(sd_event[1] - event[1]);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(spell_unit_extraction(sd_event)) === event_subject_id && difference < found_spell_min_ts) {
                    const kb_amount = get_spell_components_total_amount(sd_event[7]);
                    if (kb_amount > 0) {
                        spell_id = sd_event[5];
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }

            if (spell_id >= 0) {
                if (!newData.has(subject_id))
                    newData.set(subject_id, [get_unit_owner(death_unit_extraction(grouping[unit_id][0])), []]);
                const death_overview_rows = newData.get(subject_id)[1];
                death_overview_rows.push({
                    timestamp: event[1],
                    murder: te_death(event),
                    murdered: se_death(event),
                    killing_blow: {
                        ability_id: spell_id,
                        amount
                    }
                });
            }
        }
    }

    // @ts-ignore
    return [...newData.entries()];
}

export {commit_death};
