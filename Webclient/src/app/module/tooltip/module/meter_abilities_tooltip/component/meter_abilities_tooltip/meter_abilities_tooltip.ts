import {Component, Input} from "@angular/core";
import {Observable} from "rxjs";

@Component({
    selector: "MeterAbilitiesTooltip",
    templateUrl: "./meter_abilities_tooltip.html",
    styleUrls: ["./meter_abilities_tooltip.scss"]
})
export class MeterAbilitiesTooltipComponent {

    @Input() payload: Array<[Observable<string>, number]>;

    format_number(number_str: number): string {
        return number_str.toLocaleString("en-US");
    }

    get_percent(amount: number): string {
        return (100 * amount / this.payload.reduce((acc, value) => acc + value[1], 0)).toFixed(1);
    }
}
