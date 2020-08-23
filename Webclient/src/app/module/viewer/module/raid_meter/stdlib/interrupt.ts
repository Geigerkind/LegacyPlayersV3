import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {group_by} from "../../../../../stdlib/group_by";
import {
    get_aura_application,
    get_interrupt,
    get_spell_cast,
    get_spell_cause
} from "../../../extractor/events";
import {AuraApplication} from "../../../domain_value/aura_application";

export function commit_interrupt(interrupts: Array<Event>, event_map: Map<number, Event>, interrupt_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<UnAuraOverviewRow>]]> {
    const newData = new Map<number, [Unit, Array<UnAuraOverviewRow>]>();

    const grouping = group_by(interrupts, (event) => get_unit_id(interrupt_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [interrupt_unit_extraction(grouping[unit_id][0]), []]);
        const un_aura_overview_row = newData.get(subject_id)[1];
        grouping[subject_id].forEach(event => {
            const interrupt_event = get_interrupt(event);
            const [indicator, spell_cause_event] = get_spell_cause(interrupt_event.cause_event_id, event_map);
            const spell_cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);
            if (!!spell_cause) {
                un_aura_overview_row.push({
                    timestamp: event.timestamp,
                    caster: indicator ? spell_cause_event.subject : (spell_cause as AuraApplication).caster,
                    target: event.subject,
                    caster_spell_id: (spell_cause as any).spell_id,
                    target_spell_id: interrupt_event.interrupted_spell_id
                });
            }
        });
    }

    // @ts-ignore
    return [...newData.entries()];
}
