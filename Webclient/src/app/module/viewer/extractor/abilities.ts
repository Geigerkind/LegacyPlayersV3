import {Event} from "../domain_value/event";
import {
    get_aura_application,
    get_dispel,
    get_interrupt,
    get_spell_cast,
    get_spell_cause,
    get_spell_steal
} from "./events";
import {ce_interrupt, ce_threat} from "./causes";

function ae_spell_cast(event: Event): Array<number> {
    return [get_spell_cast(event).spell_id];
}

function ae_aura_application(event: Event): Array<number> {
    return [get_aura_application(event).spell_id];
}

function ae_interrupt(event_map: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => [get_interrupt(event).interrupted_spell_id, ...ae_spell_cast_or_aura_application(ce_interrupt, event_map)(event)];
}

function ae_spell_steal(event_map: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const spell_steal = get_spell_steal(event);
        const spell_cast_event = event_map?.get(spell_steal.cause_event_id);
        const aura_application_event = event_map?.get(spell_steal.target_event_id);
        return [(aura_application_event?.event as any)?.AuraApplication?.spell_id, (spell_cast_event?.event as any)?.SpellCast?.spell_id];
    };
}

function ae_dispel(event_map: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const dispel = get_dispel(event);
        const [indicator, spell_cause_event] = get_spell_cause(dispel.cause_event_id, event_map);
        const spell_cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);
        return [(spell_cause as any)?.spell_id, get_aura_application(event_map?.get(dispel.target_event_id))?.spell_id];
    };
}

function ae_melee_damage(event: Event): Array<number> {
    return [0];
}

function ae_threat(event_map: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const [indicator, spell_cause_event] = get_spell_cause(ce_threat(event), event_map);
        if (!!spell_cause_event)
            return [(indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event)).spell_id];
        return ae_melee_damage(event);
    };
}

function ae_spell_cast_or_aura_application(cause_extraction: (Event) => number, event_map: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const cause_event_id = cause_extraction(event);
        const [indicator, spell_cause_event] = get_spell_cause(cause_event_id, event_map);
        if (!!spell_cause_event) {
            if (indicator)
                return [get_spell_cast(spell_cause_event).spell_id];
            return [get_aura_application(spell_cause_event).spell_id];
        }
        return [];
    };
}

export function ae_spell_damage(event: any): Array<number> {
    return [event[5]];
}

export {ae_spell_cast, ae_aura_application, ae_interrupt, ae_spell_steal, ae_dispel, ae_melee_damage, ae_threat};
export {ae_spell_cast_or_aura_application};
