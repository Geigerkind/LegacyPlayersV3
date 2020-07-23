import {Unit} from "./unit";

export interface Threat {
    cause_event_id: number;
    threat: {
        threatened: Unit;
        amount: number;
    };
}
