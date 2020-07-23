import {Unit} from "./unit";
import {HitType} from "./hit_type";
import {School} from "./school";
import {Mitigation} from "./mitigation";

export interface Damage {
    school: School;
    damage: number;
    mitigation: Array<Mitigation>;
    victim: Unit;
    hit_type: HitType;
}
