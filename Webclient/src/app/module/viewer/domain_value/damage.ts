import {Unit} from "./unit";
import {HitType} from "./hit_type";
import {SpellComponent} from "./spell_component";

export interface Damage {
    victim: Unit;
    hit_mask: Array<HitType>;
    components: Array<SpellComponent>;
}
