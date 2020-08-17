import {Event} from "../domain_value/event";
import {Unit} from "../domain_value/unit";
import {AuraApplication} from "../domain_value/aura_application";
import {get_aura_application, get_dispel} from "./events";

function se_identity(event: Event): Unit {
    return event.subject;
}

function se_aura_app_or_own(cause_extraction: (Event) => number, aura_applications: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const aura_application_event = aura_applications?.get(cause_extraction(event));
        if (!!aura_application_event)
            return ((aura_application_event?.event as any)?.AuraApplication as AuraApplication)?.caster;
        return event.subject;
    };
}

function se_dispel(spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const dispel_event = get_dispel(event);
        let cause_event = spell_casts?.get(dispel_event.cause_event_id);
        if (!!cause_event)
            return cause_event.subject;
        cause_event = aura_applications?.get(dispel_event.cause_event_id);
        if (!!cause_event)
            return get_aura_application(cause_event).caster;
        return undefined;
    };
}

export {se_aura_app_or_own, se_identity, se_dispel};
