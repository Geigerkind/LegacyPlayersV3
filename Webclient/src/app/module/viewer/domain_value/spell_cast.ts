import {Unit} from "./unit";
import {HitType} from "./hit_type";
import {School} from "./school";

export interface SpellCast {
    victim: Unit;
    hit_mask: Array<HitType>;
    spell_id: number;
    school_mask: Array<School>;
}
