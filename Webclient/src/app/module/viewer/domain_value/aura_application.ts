import {Unit} from "./unit";

export interface AuraApplication {
    caster: Unit;
    stack_amount: number;
    spell_id: number;
}
