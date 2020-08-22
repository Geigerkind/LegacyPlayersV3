import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {ce_spell_damage} from "../../../extractor/causes";
import {Event} from "../../../domain_value/event";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {get_melee_damage, get_spell_damage} from "../../../extractor/events";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";
import {get_damage_components_total_damage} from "../../../domain_value/damage";

function commit_damage(melee_damage: Array<Event>, spell_damage: Array<Event>, event_map: Map<number, Event>,
                       melee_unit_extraction: (Event) => Unit, spell_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<[number, number]>]]> {
    const newData = new Map<number, [Unit, Map<number, number>]>();

    // Melee Damage
    // @ts-ignore
    let grouping = group_by([...melee_damage.values()], (event) => get_unit_id(melee_unit_extraction(event)));
    // @ts-ignore
    // tslint:disable-next-line:forin
    for (const unit_id: number in grouping) {
        const subject_id = Number(unit_id);
        const total_damage = grouping[unit_id].reduce((acc, event) => acc + get_damage_components_total_damage(get_melee_damage(event).damage_components), 0);
        if (!newData.has(subject_id))
            newData.set(subject_id, [melee_unit_extraction(grouping[unit_id][0]), new Map([[CONST_AUTO_ATTACK_ID, total_damage]])]);
        else newData.get(subject_id)[1].set(CONST_AUTO_ATTACK_ID, total_damage);
    }

    // Spell Damage
    grouping = group_by(spell_damage, (event) => get_unit_id(spell_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [spell_unit_extraction(grouping[unit_id][0]), new Map()]);

        const abilities_data = newData.get(subject_id)[1];
        grouping[subject_id].forEach(event => {
            const spell_id = ae_spell_cast_or_aura_application(ce_spell_damage, event_map)(event)[0];
            if (!spell_id)
                return;
            const damage = get_damage_components_total_damage(get_spell_damage(event).damage.damage_components);

            if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + damage);
            else abilities_data.set(spell_id, damage);
        });
    }

    // @ts-ignore
    return [...newData.entries()].map(([unit_id, [unit, abilities]]) => [unit_id, [unit, [...abilities.entries()]]]);
}

export {commit_damage};
