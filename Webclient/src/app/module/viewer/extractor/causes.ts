import {Event} from "../domain_value/event";
import {get_dispel, get_heal, get_interrupt, get_spell_damage, get_spell_steal, get_threat} from "./events";

function ce_threat(event: Event): number {
    return get_threat(event).cause_event_id;
}

function ce_interrupt(event: Event): number {
    return get_interrupt(event).cause_event_id;
}

function ce_spell_steal(event: Event): number {
    return get_spell_steal(event).cause_event_id;
}

function ce_dispel(event: Event): number {
    return get_dispel(event).cause_event_id;
}

function ce_spell_damage(event: Event): number {
    return get_spell_damage(event).spell_cause_id;
}

function ce_heal(event: Event): number {
    return get_heal(event).spell_cause_id;
}

export {ce_threat, ce_interrupt, ce_spell_steal, ce_dispel, ce_spell_damage, ce_heal};
