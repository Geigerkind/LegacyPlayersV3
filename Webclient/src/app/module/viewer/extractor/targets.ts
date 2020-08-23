import {Event} from "../domain_value/event";
import {Unit} from "../domain_value/unit";
import {
    get_aura_application, get_death, get_dispel,
    get_heal,
    get_melee_damage,
    get_spell_cast, get_spell_cause,
    get_spell_damage,
    get_summon,
    get_threat
} from "./events";

function te_spell_cast(event: Event): Unit {
    return get_spell_cast(event).victim;
}

function te_aura_application(event: Event): Unit {
    return event.subject;
}

function te_summon(event: Event): Unit {
    return get_summon(event).summoned;
}

function te_melee_damage(event: Event): Unit {
    return get_melee_damage(event).victim;
}

function te_spell_damage(event: Event): Unit {
    return get_spell_damage(event).damage.victim;
}

function te_heal(event: Event): Unit {
    return get_heal(event).heal.target;
}

function te_threat(event: Event): Unit {
    return get_threat(event).threat.threatened;
}

function te_death(event: Event): Unit {
    return get_death(event).murder;
}

function te_dispel(event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => event_map?.get(get_dispel(event).target_event_id)?.subject;
}

function te_spell_cast_by_cause(cause_extraction: (Event) => number, event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const cause_event_id = cause_extraction(event);
        const spell_cast_event = event_map?.get(cause_event_id);
        return te_spell_cast(spell_cast_event);
    };
}

function te_spell_cast_or_aura_app(cause_extraction: (Event) => number, event_map: Map<number, Event>): (Event) => Unit {
    return (event: Event) => {
        const cause_event_id = cause_extraction(event);
        const [indicator, spell_cause_event] = get_spell_cause(cause_event_id, event_map);
        if (!!spell_cause_event) {
            return indicator ? te_spell_cast(spell_cause_event) : te_aura_application(spell_cause_event);
        }
        return undefined;
    };
}

export {te_spell_cast, te_aura_application, te_summon, te_melee_damage, te_spell_damage, te_heal, te_threat, te_death, te_dispel};
export {te_spell_cast_by_cause};
export {te_spell_cast_or_aura_app};
