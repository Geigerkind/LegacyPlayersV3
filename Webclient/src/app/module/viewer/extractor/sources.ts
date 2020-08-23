import {Event} from "../domain_value/event";
import {Unit} from "../domain_value/unit";
import {
    get_aura_application,
    get_dispel,
    get_interrupt,
    get_spell_cause,
    get_spell_steal
} from "./events";

export function se_identity(event: Event): Unit {
    return event.subject;
}

export function se_aura_app_or_own(cause_extraction: (Event) => number, event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const aura_application_event = event_map?.get(cause_extraction(event));
        if (!!aura_application_event) {
            const aura_app = get_aura_application(aura_application_event);
            if (!!aura_app)
                return aura_app.caster;
        }
        return event.subject;
    };
}

function se_un_aura(un_aura_event_extractor: (Event) => any, event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const un_aura_event = un_aura_event_extractor(event);
        const [indicator, spell_cause_event] = get_spell_cause(un_aura_event.cause_event_id, event_map);
        if (!!spell_cause_event)
            return indicator ? spell_cause_event.subject : get_aura_application(spell_cause_event).caster;
        return undefined;
    };
}

export function se_dispel(event_map: Map<number, Event>): (Event) => Unit {
    return se_un_aura(get_dispel, event_map);
}

export function se_interrupt(event_map: Map<number, Event>): (Event) => Unit {
    return se_un_aura(get_interrupt, event_map);
}

export function se_spell_steal(event_map: Map<number, Event>): (Event) => Unit {
    return se_un_aura(get_spell_steal, event_map);
}
