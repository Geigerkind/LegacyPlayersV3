import {DelayedLabel} from "../../../../stdlib/delayed_label";

export interface SelectOption {
    value: number;
    label_key: string | DelayedLabel;
}
