import {Component, EventEmitter, Input, Output} from "@angular/core";
import {FormFailure} from "../../../../../material/form_failure";

@Component({
    selector: "DateInput",
    templateUrl: "./date_input.html"
})
export class DateInputComponent {
    @Input() required: boolean;
    @Input() placeholderKey: string;
    @Input() labelKey: string;
    @Input() name: string;
    @Input() formFailure: FormFailure;
    @Input() min_date: Date;
    @Input() max_date: Date;

    @Output() valueChange: EventEmitter<Date> = new EventEmitter<Date>();
    valueData: Date;

    @Input()
    get value(): Date {
        return this.valueData;
    }

    set value(newValue: Date) {
        if (this.valueData !== undefined && this.valueData.getTime() !== newValue.getTime())
            this.valueChange.emit(newValue);
        this.valueData = newValue;
    }

    parseDate(changedValue: string): void {
        this.value = new Date(changedValue);
    }

    passDate(dateVal: Date): string {
        if (!dateVal)
            return '';
        return dateVal.toISOString().slice(0, 10);
    }
}
