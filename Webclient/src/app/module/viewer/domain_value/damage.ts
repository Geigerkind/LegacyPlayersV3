import {Unit} from "./unit";
import {HitType} from "./hit_type";
import {Mitigation} from "./mitigation";
import {School} from "./school";

export interface Damage {
    victim: Unit;
    hit_mask: Array<HitType>;
    damage_components: Array<DamageComponent>;
}

export interface DamageComponent {
    school_mask: Array<School>;
    damage: number;
    mitigation: Array<Mitigation>;
}

export function get_damage_components_total_damage(components: Array<DamageComponent>): number {
    return components.reduce((acc, comp) => acc + comp.damage, 0);
}
