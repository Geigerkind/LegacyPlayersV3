import {Directive, HostListener, Input} from '@angular/core';
import {TooltipControllerService} from "../../service/tooltip_controller";

@Directive({
    selector: '[showTooltip]'
})
export class ShowTooltipDirective {

    @Input('showTooltip') tooltipArgs: any;

    constructor(
        private tooltipControllerService: TooltipControllerService
    ) {
    }

    @HostListener('mouseenter')
    onEnter(): void {
        this.tooltipControllerService.showTooltip(this.tooltipArgs);
    }

    @HostListener('mouseleave')
    onLeave(): void {
        this.tooltipControllerService.hideTooltip();
    }

    @HostListener('mousemove')
    onMove(): void {
        this.tooltipControllerService.positionTooltip();
    }
}
