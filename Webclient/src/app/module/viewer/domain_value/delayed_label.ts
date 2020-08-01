import {Observable, Subscription} from "rxjs";

export class DelayedLabel {

    private subscription: Subscription;

    content: string = "Unknown";

    constructor(content: Observable<string>) {
        this.subscription = content.subscribe(result => this.content = result);
    }
}
