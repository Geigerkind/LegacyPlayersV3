import {Observable} from "rxjs";

export interface EventSource {
    id: number;
    label: Observable<string>;
    is_player: boolean;
}
