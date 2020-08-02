import {Component, Input} from "@angular/core";
import {RaidMeterRow} from "../../../../../viewer/module/raid_meter/domain_value/raid_meter_row";

@Component({
    selector: "MeterAbilitiesTooltip",
    templateUrl: "./meter_abilities_tooltip.html",
    styleUrls: ["./meter_abilities_tooltip.scss"]
})
export class MeterAbilitiesTooltipComponent {

    @Input() payload: Array<RaidMeterRow>;

}
