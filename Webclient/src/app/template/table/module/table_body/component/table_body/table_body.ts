import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "TableBody",
    templateUrl: "./table_body.html",
    styleUrls: ["./table_body.scss"]
})
export class TableBodyComponent {

    @Input() responsiveHeadColumns: number[];
    @Input() isResponsiveMode: boolean;
    @Input() rows: BodyColumn[][];

    getResponsiveHeaderColumns(columns: BodyColumn[]): BodyColumn[] {
        return columns.filter((_, index) => this.responsiveHeadColumns.includes(index));
    }

    getResponsiveBodyColumns(columns: BodyColumn[]): BodyColumn[] {
        return columns.filter((_, index) => !this.responsiveHeadColumns.includes(index));
    }
}
