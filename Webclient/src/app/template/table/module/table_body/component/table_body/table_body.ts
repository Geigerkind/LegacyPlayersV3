import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "TableBody",
    templateUrl: "./table_body.html",
    styleUrls: ["./table_body.scss"]
})
export class TableBodyComponent {

    @Input() responsiveHeadColumns: Array<number>;
    @Input() isResponsiveMode: boolean;
    @Input() rows: Array<Array<BodyColumn>>;

    getResponsiveHeaderColumns(columns: Array<BodyColumn>): Array<BodyColumn> {
        return columns.filter((_, index) => this.responsiveHeadColumns.includes(index));
    }

    getResponsiveBodyColumns(columns: Array<BodyColumn>): Array<BodyColumn> {
        return columns.filter((_, index) => !this.responsiveHeadColumns.includes(index));
    }
}
