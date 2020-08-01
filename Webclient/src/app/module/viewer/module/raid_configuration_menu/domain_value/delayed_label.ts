import {Observable, Subscription} from "rxjs";
import {OnDestroy} from "@angular/core";

export class DelayedLabel implements OnDestroy {

    private subscription: Subscription;

    content: string = "Unknown";

    constructor(content: Observable<string>) {
        this.subscription = content.subscribe(result => this.content = result);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }
}
