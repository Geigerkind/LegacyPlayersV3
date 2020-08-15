import {Observable, Subscription} from "rxjs";

export class DelayedLabel {

    private subscription: Subscription;
    private observers: any = [];
    content: string = "Unknown";

    constructor(content: Observable<string>) {
        this.subscription = content.subscribe(result => {
            this.content = result;
            if (result !== "Unknown") {
                this.notify();
                this.subscription?.unsubscribe();
            }
        });
    }

    toString(): any {
        return this.content;
    }

    subscribe(callback: any): void {
        this.observers.push(callback);
    }

    private notify(): void {
        this.observers.forEach(callback => callback.call());
    }
}
