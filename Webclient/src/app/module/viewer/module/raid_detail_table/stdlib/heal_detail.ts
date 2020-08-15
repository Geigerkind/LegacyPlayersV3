import {SpellService} from "../../../service/spell";
import {BehaviorSubject} from "rxjs";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {Event} from "../../../domain_value/event";
import {DelayedLabel} from "../../../../../stdlib/delayed_label";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {get_aura_application, get_heal, get_spell_cast} from "../../../extractor/events";
import {map} from "rxjs/operators";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";
import {Mitigation} from "../../../domain_value/mitigation";
import {HealMode} from "../../../domain_value/heal_mode";
import {detail_row_post_processing} from "./util";

function commit_heal_detail(current_mode: HealMode, spellService: SpellService, ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>>,
                            abilities$: BehaviorSubject<Array<SelectOption>>, heal: Array<Event>,
                            spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): void {
    const abilities = new Map<number, DelayedLabel | string>();
    const ability_details = new Map<number, Map<HitType, DetailRow>>();

    for (const event of heal) {
        const heal_event = get_heal(event);
        const spell_cause_id = heal_event.spell_cause_id;
        const spell_cause_event = spell_casts.has(spell_cause_id) ? spell_casts.get(spell_cause_id) : aura_applications.get(spell_cause_id);
        if (!spell_cause_event)
            return;
        const spell_cast_event = get_spell_cast(spell_cause_event);
        const hit_type = !!spell_cast_event ? spell_cast_event.hit_type : HitType.Hit;
        const spell_id = !!spell_cast_event ? spell_cast_event.spell_id : get_aura_application(spell_cause_event).spell_id;

        if (!ability_details.has(spell_id)) {
            abilities.set(spell_id, (new DelayedLabel(spellService.get_localized_basic_spell(spell_id)
                .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
            ability_details.set(spell_id, new Map());
        }
        const details_map = ability_details.get(spell_id);
        let healing;
        if (current_mode === HealMode.Total) healing = heal_event.heal.total;
        else if (current_mode === HealMode.Effective) healing = heal_event.heal.effective;
        else healing = heal_event.heal.total - heal_event.heal.effective;
        if (current_mode !== HealMode.Overheal || healing > 0) {
            if (details_map.has(hit_type)) {
                const details = details_map.get(hit_type);
                ++details.count;
                details.amount += healing;
                details.min = Math.min(details.min, healing);
                details.max = Math.max(details.max, healing);
                details.absorb += extract_mitigation_amount(heal_event.heal.mitigation, (mitigation) => mitigation.Absorb);
            } else {
                details_map.set(hit_type, {
                    amount: healing,
                    amount_percent: 0,
                    average: 0,
                    count: 1,
                    count_percent: 0,
                    hit_type,
                    max: healing,
                    min: healing,
                    glance_or_resist: 0,
                    block: 0,
                    absorb: extract_mitigation_amount(heal_event.heal.mitigation, (mitigation) => mitigation.Absorb)
                });
            }
        }
    }

    detail_row_post_processing(ability_details);

    // @ts-ignore
    abilities$.next([...abilities.entries()].map(([value, label_key]) => {
        return {value, label_key};
    }));
    ability_details$.next(create_array_from_nested_map(ability_details));
}

function extract_mitigation_amount(mitigations: Array<Mitigation>, extract_function: (Mitigation) => number | undefined): number {
    for (const mitigation of mitigations) {
        if (extract_function(mitigation) !== undefined)
            return extract_function(mitigation) as number;
    }
    return 0;
}

export {commit_heal_detail};
