import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {ce_spell_damage} from "../../../extractor/causes";
import {Event} from "../../../domain_value/event";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {get_melee_damage, get_spell_damage} from "../../../extractor/events";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {UtilService} from "../service/util";
import {BehaviorSubject} from "rxjs";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";

function commit_damage(utilService: UtilService, data$: BehaviorSubject<Array<[number, Array<[number, number]>]>>,
                       abilities$: Map<number, RaidMeterSubject>, units$: Map<number, RaidMeterSubject>,
                       melee_damage: Map<number, Event>, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>,
                       spell_damage: Array<Event>, melee_unit_extraction: (Event) => Unit, spell_unit_extraction: (Event) => Unit): void {
    const newData = new Map();

    // Melee Damage
    // @ts-ignore
    let grouping = group_by([...melee_damage.values()], (event) => get_unit_id(melee_unit_extraction(event)));
    // @ts-ignore
    // tslint:disable-next-line:forin
    for (const unit_id: number in grouping) {
        const subject_id = Number(unit_id);

        if (!abilities$.has(CONST_AUTO_ATTACK_ID))
            abilities$.set(CONST_AUTO_ATTACK_ID, utilService.get_row_ability_subject_auto_attack());
        if (!units$.has(subject_id))
            units$.set(subject_id, utilService.get_row_unit_subject(grouping[unit_id][0].subject));

        const total_damage = grouping[unit_id].reduce((acc, event) => acc + get_melee_damage(event).damage, 0);
        if (!newData.has(subject_id))
            newData.set(subject_id, new Map([[CONST_AUTO_ATTACK_ID, total_damage]]));
        else newData.get(subject_id).set(CONST_AUTO_ATTACK_ID, total_damage);
    }

    // Spell Damage
    grouping = group_by(spell_damage, (event) => get_unit_id(spell_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!units$.has(subject_id))
            units$.set(subject_id, utilService.get_row_unit_subject(spell_unit_extraction(grouping[unit_id][0])));
        if (!newData.has(subject_id))
            newData.set(subject_id, new Map());

        const abilities_data = newData.get(subject_id);
        grouping[subject_id].forEach(event => {
            const spell_id = ae_spell_cast_or_aura_application(ce_spell_damage, spell_casts, aura_applications)(event)[0];
            if (!spell_id)
                return;
            const damage = get_spell_damage(event).damage.damage;
            if (!abilities$.has(spell_id))
                abilities$.set(spell_id, utilService.get_row_ability_subject(spell_id));

            if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + damage);
            else abilities_data.set(spell_id, damage);
        });
    }

    // @ts-ignore
    data$.next([...newData.entries()]
        .map(([unit_id, abilities]) => [unit_id, [...abilities.entries()]]));
}

export {commit_damage};
