import {PowerType} from "./power_type";

export interface Power {
    power_type: PowerType;
    max_power: number;
    current_power: number;
}
