import {Observable} from "rxjs";

export interface RaidMeterSubject {
    id: number;
    name: Observable<string>;
    color_class: Observable<string>;
    icon: Observable<string>;
}
