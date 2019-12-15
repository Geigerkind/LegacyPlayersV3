import {AfterViewInit, Directive, ElementRef, HostListener} from '@angular/core';

@Directive({
    selector: '[formValid]'
})
export class FormValidDirective implements AfterViewInit {
    private static readonly UPDATE_INTERVAL: number = 500;

    constructor(private form: ElementRef) {
        setInterval(() => this.updateValidity(), FormValidDirective.UPDATE_INTERVAL);
    }

    ngAfterViewInit(): void {
        this.updateValidity();
    }

    @HostListener('input')
    onInput(): void {
        this.updateValidity();
    }

    private updateValidity(): void {
        const isValid = this.form.nativeElement.checkValidity();
        this.setValidAttribute(isValid);
    }

    private setValidAttribute(isValid: boolean): void {
        const children = this.form.nativeElement.children;
        for (let i = 0; i < children.length; ++i) {
            const innerChildren = children[i].children;
            for (let j = 0; j < innerChildren.length; ++j) {
                if (innerChildren[j].tagName === "BUTTON" && innerChildren[j].type === "submit")
                    innerChildren[j].dataset.formValid = isValid;
            }
        }
    }
}
