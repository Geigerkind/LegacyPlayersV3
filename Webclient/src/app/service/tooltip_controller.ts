import {Injectable} from "@angular/core";
import {ObserverPattern} from "../template/class_template/observer_pattern";

@Injectable({
    providedIn: "root",
})
export class TooltipControllerService extends ObserverPattern {

    private savedReference: any;

    constructor() {
        super();
    }

    positionTooltip(isMobile: boolean, x: number, y: number): void {
        if (isMobile) this.positionMobile(y);
        else this.positionDesktop(x,y);
    }

    showTooltip(args: any, isMobile: boolean, x: number, y: number): void {
        this.positionTooltip(isMobile, x, y);
        this.getTooltip().style.display = 'block';
        this.notify(callback => callback.call(callback, args));
    }

    hideTooltip(): void {
        this.getTooltip().style.display = 'none';
        this.notify(callback => callback.call(callback, { type: undefined }));
    }

    private getTooltip(): any {
        if (this.savedReference) {
            return this.savedReference;
        }
        const tooltip = document.getElementById("global_tooltip");
        if (!!tooltip) this.savedReference = tooltip;
        return tooltip;
    }

    private positionDesktop(x: number, y: number): void {
        const tooltip = this.getTooltip();
        tooltip.style.top = y - 70 + "px";
        tooltip.style.left = x + 20 + "px";
    }

    private positionMobile(y: number): void {
        const tooltip = this.getTooltip();
        tooltip.style.top = y + 30 + "px";
        tooltip.style.left = "54px";
    }
}
