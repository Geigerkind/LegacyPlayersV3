import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {group_by} from "../../../../../stdlib/group_by";
import {
    get_aura_application,
    get_dispel,
    get_spell_cast,
    get_spell_cause,
    get_spell_steal
} from "../../../extractor/events";
import {se_dispel} from "../../../extractor/sources";

export function commit_spell_steal(spell_steals: Array<Event>, event_map: Map<number, Event>,
                                   spell_steal_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<UnAuraOverviewRow>]]> {
    const newData = new Map<number, [Unit, Array<UnAuraOverviewRow>]>();

    const grouping = group_by(spell_steals, (event) => get_unit_id(spell_steal_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [spell_steal_unit_extraction(grouping[unit_id][0]), []]);
        const un_aura_overview_row = newData.get(subject_id)[1];
        for (const event of grouping[unit_id]) {
            const spell_steal_event = get_spell_steal(event);
            const [indicator, spell_cause_event] = get_spell_cause(spell_steal_event.cause_event_id, event_map);
            const spell_cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);
            const stolen_event = event_map.get(spell_steal_event.target_event_id);
            if (!!spell_cause && !!stolen_event) {
                un_aura_overview_row.push({
                    timestamp: event.timestamp,
                    caster: se_dispel(event_map)(event),
                    target: stolen_event.subject,
                    caster_spell_id: (spell_cause as any).spell_id,
                    target_spell_id: get_aura_application(stolen_event).spell_id
                });
            }
        }
    }

    // @ts-ignore
    return [...newData.entries()];
}
