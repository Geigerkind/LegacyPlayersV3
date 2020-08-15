import {Event} from "../domain_value/event";
import {Unit} from "../domain_value/unit";
import {AuraApplication} from "../domain_value/aura_application";

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

export {se_aura_app_or_own, se_identity};
