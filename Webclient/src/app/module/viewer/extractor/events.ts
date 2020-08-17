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

function get_spell_cast(event: Event): SpellCast {
    return ((event?.event as any)?.SpellCast as SpellCast);
}

function get_aura_application(event: Event): AuraApplication {
    return ((event?.event as any)?.AuraApplication as AuraApplication);
}

function get_interrupt(event: Event): Interrupt {
    return ((event?.event as any)?.Interrupt as Interrupt);
}

function get_spell_steal(event: Event): SpellSteal {
    return ((event?.event as any)?.SpellSteal as SpellSteal);
}

function get_threat(event: Event): Threat {
    return ((event?.event as any)?.Threat as Threat);
}

function get_dispel(event: Event): Dispel {
    return ((event?.event as any)?.Dispel as Dispel);
}

function get_summon(event: Event): Summon {
    return ((event?.event as any)?.Summon as Summon);
}

function get_melee_damage(event: Event): MeleeDamage {
    return ((event?.event as any)?.MeleeDamage as MeleeDamage);
}

function get_spell_damage(event: Event): SpellDamage {
    return ((event?.event as any)?.SpellDamage as SpellDamage);
}

function get_heal(event: Event): Heal {
    return ((event?.event as any)?.Heal as Heal);
}

function get_spell_cause(spell_cause_id: number, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): [boolean, Event] {
    let cause = spell_casts?.get(spell_cause_id);
    if (!!cause)
        return [true, cause];
    cause = aura_applications?.get(spell_cause_id);
    return [false, cause];
}

function get_death(event: Event): Death {
    return ((event?.event as any)?.Death as Death);
}

export {get_spell_cast, get_aura_application, get_interrupt, get_spell_steal, get_threat, get_dispel, get_summon, get_melee_damage, get_spell_damage, get_heal, get_spell_cause, get_death};
