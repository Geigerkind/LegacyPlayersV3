import {Component, Input} from "@angular/core";
import {DetailRow} from "../../../../domain_value/detail_row";

@Component({
    selector: "DetailTable",
    templateUrl: "./detail_table.html",
    styleUrls: ["./detail_table.scss"]
})
export class DetailTableComponent {

    @Input() current_ability_details: Array<DetailRow>;

}
