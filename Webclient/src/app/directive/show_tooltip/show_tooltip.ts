import {Directive, HostListener, Input, OnDestroy} from '@angular/core';
import {TooltipControllerService} from "../../service/tooltip_controller";
import {Router} from "@angular/router";
import {Subscription} from "rxjs";

@Directive({
    selector: '[showTooltip]'
})
export class ShowTooltipDirective implements OnDestroy {

    private subscription: Subscription;
    private clicked: boolean = false;
    @Input('showTooltip') tooltipArgs: any;

    constructor(
        private tooltipControllerService: TooltipControllerService,
        private routerService: Router
    ) {
        this.subscription = this.routerService.events.subscribe(() => {
            this.clicked = false;
            this.tooltipControllerService.hideTooltip();
        });
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
        this.tooltipControllerService.showTooltip(this.tooltipArgs, false, event.clientX, event.clientY);
    }

    @HostListener('mouseleave')
    onLeave(): void {
        this.tooltipControllerService.hideTooltip();
    }

    @HostListener('mousemove', ["$event"])
    onMove(event: any): void {
        this.tooltipControllerService.positionTooltip(this.isMobile(), event.clientX, event.clientY);
    }

    private isMobile(): boolean {
        return navigator.userAgent.toLowerCase().includes("mobile");
    }
}
