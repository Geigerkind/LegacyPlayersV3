import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {Event} from "../../../domain_value/event";
import {
    get_aura_application, get_death, get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_spell_damage
} from "../../../extractor/events";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";
import {get_spell_components_total_amount} from "../../../domain_value/spell_component";

function commit_death(deaths: Array<Event>, event_map: Map<number, Event>, melee_damage: Array<Event>,
                      spell_damage: Array<Event>, melee_unit_extraction: (Event) => Unit, spell_unit_extraction: (Event) => Unit,
                      death_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<DeathOverviewRow>]]> {
    const newData = new Map<number, [Unit, Array<DeathOverviewRow>]>();

    const grouping = group_by(deaths, (event) => get_unit_id(death_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        grouping[subject_id].forEach(event => {
            // For each death find the ability that executed this death
            let spell_id;
            let found_spell_min_ts = Number.MAX_VALUE;
            let amount = 0;

            const event_subject_id = get_unit_id(event.subject);
            for (const md_event of melee_damage) {
                const difference = Math.abs(md_event.timestamp - event.timestamp);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(melee_unit_extraction(md_event)) === event_subject_id && difference < found_spell_min_ts) {
                    const kb_amount = get_spell_components_total_amount(get_melee_damage(md_event).components);
                    if (kb_amount > 0) {
                        spell_id = 0;
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }

            for (const sd_event of spell_damage) {
                const difference = Math.abs(sd_event.timestamp - event.timestamp);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(spell_unit_extraction(sd_event)) === event_subject_id && difference < found_spell_min_ts) {
                    const [indicator, cause_event] = get_spell_cause(get_spell_damage(sd_event).spell_cause_id, event_map);
                    const kb_amount = get_spell_components_total_amount(get_spell_damage(sd_event).damage.components);
                    if (kb_amount > 0) {
                        spell_id = indicator ? get_spell_cast(cause_event).spell_id : get_aura_application(cause_event).spell_id;
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }

            if (spell_id >= 0) {
                if (!newData.has(subject_id))
                    newData.set(subject_id, [death_unit_extraction(grouping[unit_id][0]), []]);
                const death_overview_rows = newData.get(subject_id)[1];
                const death_event = get_death(event);
                death_overview_rows.push({
                    timestamp: event.timestamp,
                    murder: death_event.murder,
                    murdered: event.subject,
                    killing_blow: {
                        ability_id: spell_id,
                        amount
                    }
                });
            }
        });
    }

    // @ts-ignore
    return [...newData.entries()];
}

export {commit_death};
