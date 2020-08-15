import {HitType} from "../../../domain_value/hit_type";

export interface DetailRow {
    hit_type: HitType;
    count: number;
    count_percent: number;
    amount: number;
    amount_percent: number;
    min: number;
    max: number;
    average: number;
    glance_or_resist: number;
    absorb: number;
    block: number;
}
