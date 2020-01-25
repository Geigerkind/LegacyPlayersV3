import {AfterViewInit, Component, ElementRef, EventEmitter, Input, Output, ViewChild} from "@angular/core";
import {SelectOption} from "./../../domain_value/select_option";

@Component({
    selector: "SelectInput",
    templateUrl: "./select_input.html",
    styleUrls: ["./select_input.scss"]
})
export class SelectInputComponent implements AfterViewInit {

    @ViewChild("selectInput", {static: true}) inputRef: ElementRef;
    @Input() options: Array<SelectOption>;
    @Input() autoFocus: boolean = false;

    @Output() valueChange: EventEmitter<number> = new EventEmitter<number>();
    valueData: number;
    @Input()
    get value(): number {
        return this.valueData;
    }

    set value(selectedOption: number) {
        selectedOption = Number(selectedOption);
        if (this.valueData !== selectedOption) {
            this.valueChange.emit(selectedOption);
        }
        this.valueData = selectedOption;
    }

    ngAfterViewInit(): void {
        if (this.autoFocus) {
            this.inputRef.nativeElement.focus();
        }
    }

}
