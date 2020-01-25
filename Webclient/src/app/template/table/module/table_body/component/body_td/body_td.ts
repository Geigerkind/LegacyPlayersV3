import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";
import {DatePipe} from "@angular/common";

@Component({
    selector: "BodyTd",
    templateUrl: "./body_td.html",
    styleUrls: ["./body_td.scss"],
    providers: [DatePipe]
})
export class BodyTdComponent {
    @Input() specification: BodyColumn;

    constructor(
        private datePipe: DatePipe
    ) {}

    bodyContentToString(): string {
        if (this.specification.type === 2)
            return this.datePipe.transform(new Date(Number(this.specification.content)), 'dd.MM.yyyy');
        return this.specification.content;
    }
}
