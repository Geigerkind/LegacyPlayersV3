import {Injectable} from "@angular/core";
import {NavigationCancel, NavigationEnd, NavigationError, NavigationStart, Router} from "@angular/router";
import {Observable, Subject} from "rxjs";

@Injectable({
    providedIn: "root",
})
export class LoadingBarService {

    private isLoading$: Subject<boolean> = new Subject();
    private openRequests = 0;

    constructor(private routerService: Router) {
        this.routerService.events.subscribe(event => {
            switch (true) {
                case event instanceof NavigationStart: {
                    this.incrementCounter();
                    break;
                }
                case event instanceof NavigationEnd:
                case event instanceof NavigationCancel:
                case event instanceof NavigationError: {
                    this.decrementCounter();
                    break;
                }
                default:
                    break;
            }
        });
    }

    get loading(): Observable<boolean> {
        return this.isLoading$.asObservable();
    }

    isLoading(): boolean {
        return this.openRequests > 0;
    }

    incrementCounter(): void {
        ++this.openRequests;
        if (this.openRequests === 1)
            this.propagate();
    }

    decrementCounter(): void {
        --this.openRequests;
        if (this.openRequests <= 0) {
            this.openRequests = 0;
            this.propagate();
        }
    }

    private propagate(): void {
        this.isLoading$.next(this.isLoading());
    }

}
