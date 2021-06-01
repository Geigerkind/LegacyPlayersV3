import {Component, Input} from "@angular/core";
import {DateService} from "../../../../../../service/date";

@Component({
    selector: "RankingMeterTooltip",
    templateUrl: "./ranking_meter_tooltip.html",
    styleUrls: ["./ranking_meter_tooltip.scss"]
})
export class RankingMeterTooltipComponent {

    @Input() payload: {
        encounter_names: Array<string>,
        amounts: Array<number>,
        durations: Array<number>
    };

    constructor(
        public dateService: DateService
    ) {
    }

    format_number(number_str: number): string {
        return number_str.toLocaleString("en-US");
    }

    get results(): Array<[string, number, number, number]> {
        const results = [];
        for (let i=0; i<this.payload.encounter_names.length; ++i) {
            results.push([
                this.payload.encounter_names[i],
                this.payload.amounts[i],
                this.payload.durations[i],
                (this.payload.amounts[i] / (this.payload.durations[i] / 1000)).toFixed(1)
            ]);
        }
        return results.sort((left, right) => right[3] - left[3]);
    }
}
