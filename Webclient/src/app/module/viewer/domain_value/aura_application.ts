import {Unit} from "./unit";
import {School} from "./school";

export interface AuraApplication {
    caster: Unit;
    stack_amount: number;
    spell_id: number;
    school_mask: Array<School>;
}
