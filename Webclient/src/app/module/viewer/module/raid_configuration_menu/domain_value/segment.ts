import {DelayedLabel} from "./delayed_label";

export interface Segment {
    id: number;
    label: DelayedLabel;
    duration: number;
    is_kill: boolean;
    start_ts: number;
}
