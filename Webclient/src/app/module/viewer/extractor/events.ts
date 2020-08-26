import {Event} from "../domain_value/event";
import {SpellCast} from "../domain_value/spell_cast";
import {AuraApplication} from "../domain_value/aura_application";
import {Interrupt} from "../domain_value/interrupt";
import {SpellSteal} from "../domain_value/spell_steal";
import {Threat} from "../domain_value/threat";
import {Dispel} from "../domain_value/dispel";
import {Summon} from "../domain_value/summon";
import {MeleeDamage} from "../domain_value/melee_damage";
import {SpellDamage} from "../domain_value/spell_damage";
import {Heal} from "../domain_value/heal";
import {Death} from "../domain_value/death";
import {CombatState} from "../domain_value/combat_state";

export function get_spell_cast(event: Event): SpellCast {
    return ((event?.event as any)?.SpellCast as SpellCast);
}

export function get_aura_application(event: Event): AuraApplication {
    return ((event?.event as any)?.AuraApplication as AuraApplication);
}

export function get_interrupt(event: Event): Interrupt {
    return ((event?.event as any)?.Interrupt as Interrupt);
}

export function get_spell_steal(event: Event): SpellSteal {
    return ((event?.event as any)?.SpellSteal as SpellSteal);
}

export function get_threat(event: Event): Threat {
    return ((event?.event as any)?.Threat as Threat);
}

export function get_dispel(event: Event): Dispel {
    return ((event?.event as any)?.Dispel as Dispel);
}

export function get_summon(event: Event): Summon {
    return ((event?.event as any)?.Summon as Summon);
}

export function get_melee_damage(event: Event): MeleeDamage {
    return ((event?.event as any)?.MeleeDamage as MeleeDamage);
}

export function get_spell_damage(event: Event): SpellDamage {
    return ((event?.event as any)?.SpellDamage as SpellDamage);
}

export function get_heal(event: Event): Heal {
    return ((event?.event as any)?.Heal as Heal);
}

export function get_combat_state(event: Event): CombatState {
    return ((event?.event as any)?.CombatState as CombatState);
}

export function get_spell_cause(spell_cause_id: number, event_map: Map<number, Event>): [boolean, Event] {
    const cause_event = event_map?.get(spell_cause_id);
    const is_spell_cast = !!get_spell_cast(cause_event);
    if (!!cause_event && (is_spell_cast || !!get_aura_application(cause_event)))
        return [is_spell_cast, cause_event];
    return [undefined, undefined];
}

export function get_death(event: Event): Death {
    return ((event?.event as any)?.Death as Death);
}
