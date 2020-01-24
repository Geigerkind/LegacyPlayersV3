import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "BodyTd",
    templateUrl: "./body_td.html",
    styleUrls: ["./body_td.scss"]
})
export class BodyTdComponent {
    @Input() specification: BodyColumn;
}
