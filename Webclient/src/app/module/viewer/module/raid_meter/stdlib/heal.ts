import {UtilService} from "../service/util";
import {BehaviorSubject} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_heal} from "../../../extractor/events";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {ce_heal} from "../../../extractor/causes";
import {HealMode} from "../../../domain_value/heal_mode";

function commit_heal(current_mode: HealMode, utilService: UtilService, data$: BehaviorSubject<Array<[number, Array<[number, number]>]>>,
                     abilities$: Map<number, RaidMeterSubject>, units$: Map<number, RaidMeterSubject>,
                     spell_casts: Map<number, Event>, aura_applications: Map<number, Event>, heal: Array<Event>, heal_unit_extraction: (Event) => Unit): void {
    const newData = new Map();

    // Heal
    const grouping = group_by(heal, (event) => get_unit_id(heal_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!units$.has(subject_id))
            units$.set(subject_id, utilService.get_row_unit_subject(heal_unit_extraction(grouping[unit_id][0])));
        if (!newData.has(subject_id))
            newData.set(subject_id, new Map());

        const abilities_data = newData.get(subject_id);
        grouping[subject_id].forEach(event => {
            const spell_id = ae_spell_cast_or_aura_application(ce_heal, spell_casts, aura_applications)(event)[0];
            if (!spell_id)
                return;
            let healing;
            if (current_mode === HealMode.Total) healing = get_heal(event).heal.total;
            else if (current_mode === HealMode.Effective) healing = get_heal(event).heal.effective;
            else healing = get_heal(event).heal.total - get_heal(event).heal.effective;
            if (current_mode !== HealMode.Overheal || healing > 0) {
                if (!abilities$.has(spell_id))
                    abilities$.set(spell_id, utilService.get_row_ability_subject(spell_id));

                if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + healing);
                else abilities_data.set(spell_id, healing);
            }
        });
    }

    // @ts-ignore
    data$.next([...newData.entries()]
        .map(([unit_id, abilities]) => [unit_id, [...abilities.entries()]]));
}

export {commit_heal};
