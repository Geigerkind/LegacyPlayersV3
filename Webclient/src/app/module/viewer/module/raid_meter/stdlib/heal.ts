import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_heal} from "../../../extractor/events";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {ce_heal} from "../../../extractor/causes";
import {HealMode} from "../../../domain_value/heal_mode";

function commit_heal(current_mode: HealMode, heal: Array<Event>, event_map: Map<number, Event>, heal_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<[number, number]>]]> {
    const newData = new Map<number, [Unit, Map<number, number>]>();

    // Heal
    const grouping = group_by(heal, (event) => get_unit_id(heal_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [heal_unit_extraction(grouping[unit_id][0]), new Map()]);

        const abilities_data = newData.get(subject_id)[1];
        for (const event of grouping[unit_id]) {
            const spell_id = ae_spell_cast_or_aura_application(ce_heal, event_map)(event)[0];
            let healing;
            if (current_mode === HealMode.Total) healing = get_heal(event).heal.total;
            else if (current_mode === HealMode.Effective) healing = get_heal(event).heal.effective;
            else healing = get_heal(event).heal.total - get_heal(event).heal.effective;
            if (current_mode !== HealMode.Overheal || healing > 0) {
                if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + healing);
                else abilities_data.set(spell_id, healing);
            }
        }
    }

    // @ts-ignore
    return [...newData.entries()].map(([unit_id, [unit, abilities]]) => [unit_id, [unit, [...abilities.entries()]]]);
}

export {commit_heal};
