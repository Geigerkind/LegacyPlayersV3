import {Observable} from "rxjs";
import {take} from "rxjs/operators";

export class DelayedLabel {
    content: string = "Unknown";

    constructor(content: Observable<string>) {
        content.pipe(take(1))
            .subscribe(result => this.content = result);
    }
}
