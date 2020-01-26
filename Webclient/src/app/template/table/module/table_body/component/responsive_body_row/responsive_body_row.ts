import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "ResponsiveBodyRow",
    templateUrl: "./responsive_body_row.html",
    styleUrls: ["./responsive_body_row.scss"]
})
export class ResponsiveBodyRowComponent {
    @Input() responsiveHeadColumns: Array<number>;
    @Input() columns: Array<BodyColumn>;
    @Input() typeRange: Map<number, string>[];

    isVisible: boolean = false;

    toggleVisibility(): void {
        this.isVisible = !this.isVisible;
    }

    getResponsiveHeaderColumns(): any {
        return this.columns.map((columns, index) => { return { i: index, c: columns }; }).filter(result => this.responsiveHeadColumns.includes(result.i));
    }

    getResponsiveBodyColumns(): any {
        return this.columns.map((columns, index) => { return { i: index, c: columns }; }).filter(result => !this.responsiveHeadColumns.includes(result.i));
    }
}
