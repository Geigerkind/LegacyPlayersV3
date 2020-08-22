import {Mitigation} from "./mitigation";
import {Unit} from "./unit";
import {HitType} from "./hit_type";

export interface Heal {
    spell_cause_id: number;
    heal: {
        total: number;
        effective: number;
        mitigation: Array<Mitigation>;
        target: Unit;
        hit_mask: Array<HitType>;
    };
}
