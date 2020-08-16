import {Observable} from "rxjs";

export interface EventLogEntry {
    id: number;
    timestamp: number;
    event_message: Observable<string>;
}
