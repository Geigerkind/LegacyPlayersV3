import {Event} from "../domain_value/event";
import {get_aura_application, get_dispel, get_interrupt, get_spell_cast, get_spell_steal} from "./events";
import {ce_interrupt, ce_threat} from "./causes";
import {Threat} from "../domain_value/threat";

function ae_spell_cast(event: Event): Array<number> {
    return [get_spell_cast(event).spell_id];
}

function ae_aura_application(event: Event): Array<number> {
    return [get_aura_application(event).spell_id];
}

function ae_interrupt(spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => [get_interrupt(event).interrupted_spell_id, ...ae_spell_cast_or_aura_application(ce_interrupt, spell_casts, aura_applications)(event)];
}

function ae_spell_steal(spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const spell_steal = get_spell_steal(event);
        const spell_cast_event = spell_casts?.get(spell_steal.cause_event_id);
        const aura_application_event = aura_applications?.get(spell_steal.target_event_id);
        return [(aura_application_event?.event as any)?.AuraApplication?.spell_id, (spell_cast_event?.event as any)?.SpellCast?.spell_id];
    };
}

function ae_dispel(spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const dispel = get_dispel(event);
        const spell_cast_event = spell_casts?.get(dispel.cause_event_id);
        const result = [(spell_cast_event?.event as any)?.SpellCast?.spell_id];
        dispel.target_event_ids.forEach(target_event_id =>
            result.push((aura_applications?.get(target_event_id).event as any).AuraApplication.spell_id));
        return result;
    };
}

function ae_melee_damage(event: Event): Array<number> {
    return [0];
}

function ae_threat(spell_casts: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const spell_cast_event = spell_casts?.get(ce_threat(event));
        if (!!spell_cast_event)
            return [(spell_cast_event.event as any).SpellCast.spell_id];
        return ae_melee_damage(event);
    };
}

function ae_spell_cast_or_aura_application(cause_extraction: (Event) => number, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): (Event) => Array<number> {
    return (event: Event) => {
        const cause_event_id = cause_extraction(event);
        const spell_cast_event = spell_casts?.get(cause_event_id);
        if (!!spell_cast_event)
            return [(spell_cast_event.event as any).SpellCast.spell_id];
        const aura_application_event = aura_applications?.get(cause_event_id);
        if (!!aura_application_event)
            return [(aura_application_event.event as any).AuraApplication.spell_id];
        return [];
    };
}

export {ae_spell_cast, ae_aura_application, ae_interrupt, ae_spell_steal, ae_dispel, ae_melee_damage, ae_threat};
export {ae_spell_cast_or_aura_application};
