import {Component, Input} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "HeaderRow",
    templateUrl: "./header_row.html",
    styleUrls: ["./header_row.scss"]
})
export class HeaderRowComponent {

    @Input() columns: HeaderColumn[];

}
