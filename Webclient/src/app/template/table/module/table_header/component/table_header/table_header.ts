import {Component, EventEmitter, Input, Output} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "TableHeader",
    templateUrl: "./table_header.html",
    styleUrls: ["./table_header.scss"]
})
export class TableHeaderComponent {

    @Input() responsiveHeadColumns: number[];
    @Input() isResponsiveMode: boolean;
    @Input() columns: HeaderColumn[];

    @Output() filterChanged: EventEmitter<object> = new EventEmitter<object>();

    getResponsiveHeaderColumns(columns: HeaderColumn[]): HeaderColumn[] {
        return columns.filter((_, index) => this.responsiveHeadColumns.includes(index));
    }

    getResponsiveBodyColumns(columns: HeaderColumn[]): HeaderColumn[] {
        return columns.filter((_, index) => !this.responsiveHeadColumns.includes(index));
    }

    bubbleFilter(filter: object): void {
        this.filterChanged.emit(filter);
    }
}
