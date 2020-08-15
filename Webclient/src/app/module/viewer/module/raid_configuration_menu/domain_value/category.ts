import {DelayedLabel} from "../../../../../stdlib/delayed_label";

export interface Category {
    label: DelayedLabel;
    id: number;
    time: number;
    segments: Set<number>;
}
