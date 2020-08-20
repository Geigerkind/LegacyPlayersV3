import {Observable} from "rxjs";

export interface SelectOption {
    value: number;
    label_key: string | Observable<string>;
}
