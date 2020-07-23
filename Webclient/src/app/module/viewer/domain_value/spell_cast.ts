import {Unit} from "./unit";
import {HitType} from "./hit_type";

export interface SpellCast {
    victim: Unit;
    hit_type: HitType;
    spell_id: number;
}
