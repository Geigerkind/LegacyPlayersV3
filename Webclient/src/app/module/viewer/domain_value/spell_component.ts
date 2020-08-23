import {School} from "./school";
import {Mitigation} from "./mitigation";

export interface SpellComponent {
    school_mask: Array<School>;
    amount: number;
    mitigation: Array<Mitigation>;
}

export function get_spell_components_total_amount(components: Array<SpellComponent>): number {
    return components.reduce((acc, comp) => acc + comp.amount, 0);
}
