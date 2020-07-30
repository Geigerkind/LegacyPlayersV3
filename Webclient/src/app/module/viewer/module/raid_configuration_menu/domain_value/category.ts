import {Observable} from "rxjs";

export interface Category {
    id: number;
    label: Observable<string>;
    time: number;
    segments: Set<number>;
}
