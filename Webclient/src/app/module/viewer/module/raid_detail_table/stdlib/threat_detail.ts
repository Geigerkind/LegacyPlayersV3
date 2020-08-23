import {Event} from "../../../domain_value/event";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow, DetailRowContent} from "../domain_value/detail_row";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_threat
} from "../../../extractor/events";
import {detail_row_post_processing, fill_details} from "./util";
import {School} from "../../../domain_value/school";
import {get_spell_components_total_amount, SpellComponent} from "../../../domain_value/spell_component";

function commit_threat_detail(threat: Array<Event>, event_map: Map<number, Event>): Array<[number, Array<[HitType, DetailRow]>]> {
    const ability_details = new Map<number, Map<HitType, [DetailRowContent, Map<School, [DetailRowContent, Array<number>]>]>>();

    for (const event of threat) {
        const threat_event = get_threat(event);
        const [indicator, spell_cause_event] = get_spell_cause(threat_event.cause_event_id, event_map);

        let hit_mask;
        let spell_id;
        const components: Array<SpellComponent> = [];
        const threat_amount = threat_event.threat.amount;
        if (!spell_cause_event) {
            const melee_damage_event = event_map.get(threat_event.cause_event_id);
            if (!melee_damage_event)
                return;

            const melee_damage = get_melee_damage(melee_damage_event);
            hit_mask = melee_damage.hit_mask;
            const total_comp_amount = get_spell_components_total_amount(melee_damage.components);
            const scaling_factor = (threat_amount / total_comp_amount);
            for (const comp of melee_damage.components) {
                components.push({
                    amount: Math.round(threat_amount * (comp.amount / total_comp_amount)),
                    school_mask: comp.school_mask,
                    // @ts-ignore
                    mitigation: comp.mitigation.map(mitigation => {
                        if (!!(mitigation as any).Glance)
                            return {Glance: Math.round(scaling_factor * (mitigation as any).Glance)};
                        if (!!(mitigation as any).Resist)
                            return {Resist: Math.round(scaling_factor * (mitigation as any).Resist)};
                        if (!!(mitigation as any).Absorb)
                            return {Absorb: Math.round(scaling_factor * (mitigation as any).Absorb)};
                        return {Block: Math.round(scaling_factor * (mitigation as any).Block)};
                    })
                });
            }

            spell_id = 0;
        } else {
            hit_mask = indicator ? get_spell_cast(spell_cause_event).hit_mask : [HitType.Hit];
            spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
            const school_mask = indicator ? get_spell_cast(spell_cause_event).school_mask : get_aura_application(spell_cause_event).school_mask;
            components.push({
                school_mask,
                amount: threat_amount,
                mitigation: [] // TODO: If we would reference to the spell damage or heal we could also add mitigations here
            });
        }

        if (!ability_details.has(spell_id))
            ability_details.set(spell_id, new Map());

        const details_map = ability_details.get(spell_id);

        fill_details(components, hit_mask, details_map);
    }

    return detail_row_post_processing(ability_details);
}

export {commit_threat_detail};
