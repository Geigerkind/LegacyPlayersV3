import {Observable} from "rxjs";

export interface Segment {
    id: number;
    label: Observable<string>;
    duration: number;
    is_kill: boolean;
    start_ts: number;
}
