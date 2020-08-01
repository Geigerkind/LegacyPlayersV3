import {DelayedLabel} from "../../../domain_value/delayed_label";

export interface EventSource {
    id: number;
    label: DelayedLabel;
    is_player: boolean;
}
