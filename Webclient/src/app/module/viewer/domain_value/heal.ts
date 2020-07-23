import {Mitigation} from "./mitigation";
import {Unit} from "./unit";

export interface Heal {
    spell_cast_id: number;
    heal: {
        total: number;
        effective: number;
        mitigation: Array<Mitigation>;
        target: Unit;
    };
}
