import {Directive, HostListener, Input, OnDestroy} from '@angular/core';
import {TooltipControllerService} from "../../service/tooltip_controller";
import {Router} from "@angular/router";
import {Subject, Subscription} from "rxjs";
import {debounceTime} from "rxjs/operators";

@Directive({
    selector: '[showTooltip]'
})
export class ShowTooltipDirective implements OnDestroy {

    private subscription: Subscription;
    private clicked: boolean = false;
    private visibility$: Subject<[boolean, any]> = new Subject();

    @Input('showTooltip') tooltipArgs: any;

    constructor(
        private tooltipControllerService: TooltipControllerService,
        private routerService: Router
    ) {
        this.subscription = this.routerService.events.subscribe(() => {
            this.clicked = false;
            this.tooltipControllerService.hideTooltip();
        });
        this.subscription.add(this.visibility$.pipe(debounceTime(50)).subscribe(([show_tooltip, args]) => {
            if (show_tooltip) this.tooltipControllerService.showTooltip(this.tooltipArgs, false, args[0], args[1]);
            else this.tooltipControllerService.hideTooltip();
        }));
    }

    ngOnDestroy(): void {
        this.tooltipControllerService.hideTooltip();
        this.subscription?.unsubscribe();
    }

    @HostListener('click', ["$event"])
    onClick(event: any): void {
        if (!this.isMobile())
            return;

        if (!this.clicked) this.tooltipControllerService.showTooltip(this.tooltipArgs, true, event.clientX, event.clientY);
        else this.tooltipControllerService.hideTooltip();
        this.clicked = !this.clicked;
    }

    @HostListener('mouseenter', ["$event"])
    onEnter(event: any): void {
        this.visibility$.next([true, [event.clientX, event.clientY]]);
    }

    @HostListener('mouseleave')
    onLeave(): void {
        this.visibility$.next([false, undefined]);
    }

    @HostListener('mousemove', ["$event"])
    onMove(event: any): void {
        this.tooltipControllerService.positionTooltip(this.isMobile(), event.clientX, event.clientY + document.getElementsByTagName("body")[0].scrollTop);
    }

    private isMobile(): boolean {
        return navigator.userAgent.toLowerCase().includes("mobile");
    }
}
