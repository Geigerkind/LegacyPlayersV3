import {Directive, HostListener, Input} from '@angular/core';
import {TooltipControllerService} from "../../service/tooltip_controller";
import {Router} from "@angular/router";

@Directive({
    selector: '[showTooltip]'
})
export class ShowTooltipDirective {

    private clicked: boolean = false;
    @Input('showTooltip') tooltipArgs: any;

    constructor(
        private tooltipControllerService: TooltipControllerService,
        private routerService: Router
    ) {
        this.routerService.events.subscribe(() => {
            this.clicked = false;
            this.tooltipControllerService.hideTooltip();
        });
    }

    @HostListener('click')
    onClick(): void {
        if (!this.isMobile())
            return;

        if (!this.clicked) this.tooltipControllerService.showTooltip(this.tooltipArgs, true);
        else this.tooltipControllerService.hideTooltip();
        this.clicked = !this.clicked;
    }

    @HostListener('mouseenter')
    onEnter(): void {
        this.tooltipControllerService.showTooltip(this.tooltipArgs, false);
    }

    @HostListener('mouseleave')
    onLeave(): void {
        this.tooltipControllerService.hideTooltip();
    }

    @HostListener('mousemove')
    onMove(): void {
        this.tooltipControllerService.positionTooltip(this.isMobile());
    }

    private isMobile(): boolean {
        return navigator.userAgent.toLowerCase().includes("mobile");
    }
}
