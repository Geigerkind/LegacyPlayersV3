import {Component, Input} from "@angular/core";
import {DetailRow} from "../../../../../viewer/module/raid_detail_table/domain_value/detail_row";

@Component({
    selector: "MeterDetailsTooltip",
    templateUrl: "./meter_details_tooltip.html",
    styleUrls: ["./meter_details_tooltip.scss"]
})
export class MeterDetailsTooltipComponent {

    @Input() payload: Array<[string, DetailRow]>;
}
