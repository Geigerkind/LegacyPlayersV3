import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_threat} from "../../../extractor/events";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {ce_threat} from "../../../extractor/causes";

function commit_threat(threats: Array<Event>, event_map: Map<number, Event>, threat_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<[number, number]>]]> {
    const newData = new Map<number, [Unit, Map<number, number>]>();

    const grouping = group_by(threats, (event) => get_unit_id(threat_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [threat_unit_extraction(grouping[unit_id][0]), new Map()]);

        const abilities_data = newData.get(subject_id)[1];
        grouping[subject_id].forEach(event => {
            let spell_id = ae_spell_cast_or_aura_application(ce_threat, event_map)(event)[0];
            if (!spell_id)
                spell_id = CONST_AUTO_ATTACK_ID;
            const threat = get_threat(event).threat.amount;
            if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + threat);
            else abilities_data.set(spell_id, threat);
        });
    }

    // @ts-ignore
    return [...newData.entries()].map(([unit_id, [unit, abilities]]) => [unit_id, [unit, [...abilities.entries()]]]);
}

export {commit_threat};
