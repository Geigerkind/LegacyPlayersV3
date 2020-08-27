import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_aura_application, get_dispel, get_spell_cast, get_spell_cause} from "../../../extractor/events";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {se_dispel} from "../../../extractor/sources";

function commit_dispel(dispels: Array<Event>, event_map: Map<number, Event>, dispel_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<UnAuraOverviewRow>]]> {
    const newData = new Map<number, [Unit, Array<UnAuraOverviewRow>]>();

    const grouping = group_by(dispels, (event) => get_unit_id(dispel_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [dispel_unit_extraction(grouping[unit_id][0]), []]);
        const un_aura_overview_row = newData.get(subject_id)[1];
        for (const event of grouping[unit_id]) {
            const dispel_event = get_dispel(event);
            const [indicator, spell_cause_event] = get_spell_cause(dispel_event.cause_event_id, event_map);
            const spell_cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);
            const dispelled_aura_event = event_map.get(dispel_event.target_event_id);
            if (!!spell_cause && !!dispelled_aura_event) {
                un_aura_overview_row.push({
                    timestamp: event.timestamp,
                    caster: se_dispel(event_map)(event),
                    target: dispelled_aura_event.subject,
                    caster_spell_id: (spell_cause as any).spell_id,
                    target_spell_id: get_aura_application(dispelled_aura_event).spell_id
                });
            }
        }
    }

    // @ts-ignore
    return [...newData.entries()];
}

export {commit_dispel};
