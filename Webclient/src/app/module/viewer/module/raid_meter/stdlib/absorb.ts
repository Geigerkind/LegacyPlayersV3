import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_melee_damage, get_spell_damage} from "../../../extractor/events";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {ce_spell_damage} from "../../../extractor/causes";
import {HitType} from "../../../domain_value/hit_type";
import {extract_mitigation_amount} from "../../raid_detail_table/stdlib/util";
import {School} from "../../../domain_value/school";

export function commit_absorb_damages(melee_damage: Array<Event>, spell_damage: Array<Event>, event_map: Map<number, Event>,
                                      melee_unit_extraction: (Event) => Unit, spell_unit_extraction: (Event) => Unit): Array<[number, [Unit, Array<[number, number, number, Array<School>]>]]> {
    const newData = new Map<number, [Unit, Array<[number, number, number, Array<School>]>]>();

    // Melee Damage
    // @ts-ignore
    let grouping = group_by([...melee_damage.values()], (event) => get_unit_id(melee_unit_extraction(event)));
    // @ts-ignore
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        const result = [];
        for (const event of grouping[unit_id]) {
            const melee_damage_event = get_melee_damage(event);
            if (!melee_damage_event.hit_mask.includes(HitType.FullAbsorb) && !melee_damage_event.hit_mask.includes(HitType.PartialAbsorb))
                continue;
            for (const component of melee_damage_event.components) {
                const absorb = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Absorb);
                if (absorb === 0)
                    continue;
                result.push([CONST_AUTO_ATTACK_ID, event.timestamp, absorb, component.school_mask]);
            }
        }
        if (result.length > 0)
            newData.set(subject_id, [melee_unit_extraction(grouping[unit_id][0]), result]);
    }

    // Spell Damage
    grouping = group_by(spell_damage, (event) => get_unit_id(spell_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [spell_unit_extraction(grouping[unit_id][0]), []]);

        const absorbs = newData.get(subject_id)[1];
        for (const event of grouping[unit_id]) {
            const spell_damage_event = get_spell_damage(event);
            const spell_id = ae_spell_cast_or_aura_application(ce_spell_damage, event_map)(event)[0];
            if (!spell_damage_event.damage.hit_mask.includes(HitType.FullAbsorb) && !spell_damage_event.damage.hit_mask.includes(HitType.PartialAbsorb))
                continue;
            for (const component of spell_damage_event.damage.components) {
                const absorb = extract_mitigation_amount(component.mitigation, (mitigation) => mitigation.Absorb);
                if (absorb === 0)
                    continue;
                absorbs.push([spell_id, event.timestamp, absorb, component.school_mask]);
            }
        }

        if (absorbs.length === 0)
            newData.delete(subject_id);
    }

    // @ts-ignore
    return [...newData.entries()];
}
