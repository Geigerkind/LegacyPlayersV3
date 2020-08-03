import {Component, Input} from "@angular/core";
import {Observable} from "rxjs";

@Component({
    selector: "MeterAbilitiesTooltip",
    templateUrl: "./meter_abilities_tooltip.html",
    styleUrls: ["./meter_abilities_tooltip.scss"]
})
export class MeterAbilitiesTooltipComponent {

    @Input() payload: Array<[Observable<string>, number]>;

}
