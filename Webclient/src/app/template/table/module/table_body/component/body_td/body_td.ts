import {Component, Input, TemplateRef} from "@angular/core";
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
    @Input() typeRange: Map<number, string>;
    @Input() et_item: TemplateRef<any>;

    constructor(
        private datePipe: DatePipe
    ) {}

    bodyContentToString(): string {
        if (this.specification.type === 2)
            return this.datePipe.transform(new Date(Number(this.specification.content) * 1000), 'dd.MM.yyyy');
        if (this.specification.type === 3 && this.typeRange.has(Number(this.specification.content)))
            return this.typeRange.get(Number(this.specification.content));
        return this.specification.content;
    }
}
