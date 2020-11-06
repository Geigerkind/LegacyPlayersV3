import {Observable} from "rxjs";

export interface EventLogEntry {
    id: number;
    type: number;
    timestamp: number;
    subject_id: number;
    event_message: Observable<string>;
}
