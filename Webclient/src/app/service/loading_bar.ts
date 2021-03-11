import {Injectable} from "@angular/core";
import {NavigationCancel, NavigationEnd, NavigationError, NavigationStart, Router} from "@angular/router";
import {Observable, Subject} from "rxjs";
import {auditTime} from "rxjs/operators";

@Injectable({
    providedIn: "root",
})
export class LoadingBarService {

    private isLoading$: Subject<boolean> = new Subject();
    private update$: Subject<void> = new Subject();
    private openRequests: number = 0;
    private prevLoading: boolean = false;

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
        this.update$.pipe(auditTime(500)).subscribe(() => {
            const loading = this.isLoading();
            if (!loading && this.prevLoading) {
                this.prevLoading = false;
                this.isLoading$.next(false);
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
        const loading = this.isLoading();
        if (loading && !this.prevLoading) {
            this.prevLoading = true;
            this.isLoading$.next(true);
        } else {
            this.update$.next();
        }
    }

}
