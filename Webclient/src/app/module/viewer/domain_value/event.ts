import {Unit} from "./unit";
import {EventType} from "./event_type";

export interface Event {
    id: number;
    timestamp: number;
    subject: Unit;
    event: EventType;
}
