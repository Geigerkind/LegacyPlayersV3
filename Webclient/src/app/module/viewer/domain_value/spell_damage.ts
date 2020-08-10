import {Damage} from "./damage";

export interface SpellDamage {
    spell_cause_id: number;
    damage: Damage;
}
