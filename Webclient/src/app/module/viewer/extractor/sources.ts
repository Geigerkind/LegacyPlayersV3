import {Event} from "../domain_value/event";
import {Unit} from "../domain_value/unit";
import {AuraApplication} from "../domain_value/aura_application";
import {get_aura_application, get_dispel, get_interrupt, get_spell_cast, get_spell_cause} from "./events";
import {te_aura_application, te_spell_cast} from "./targets";

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

export function se_dispel(event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const dispel_event = get_dispel(event);
        const [indicator, spell_cause_event] = get_spell_cause(dispel_event.cause_event_id, event_map);
        if (!!spell_cause_event)
            return indicator ? spell_cause_event.subject : get_aura_application(spell_cause_event).caster;
        return undefined;
    };
}

export function se_interrupt(event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const interrupt_event = get_interrupt(event);
        const [indicator, spell_cause_event] = get_spell_cause(interrupt_event.cause_event_id, event_map);
        if (!!spell_cause_event) {
            return indicator ? se_identity(spell_cause_event) : get_aura_application(spell_cause_event).caster;
        }
        return undefined;
    };
}
