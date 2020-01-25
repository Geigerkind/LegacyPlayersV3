import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "ResponsiveHeaderRow",
    templateUrl: "./responsive_header_row.html",
    styleUrls: ["./responsive_header_row.scss"]
})
export class ResponsiveHeaderRowComponent implements OnInit {

    @Input() responsiveHeaderColumns: HeaderColumn[];
    @Input() responsiveBodyColumns: HeaderColumn[];
    @Output() filterChanged: EventEmitter<object> = new EventEmitter<object>();

    isVisible: boolean = false;
    currentFilter: object = {};

    ngOnInit(): void {
        this.responsiveHeaderColumns.forEach(item => {
            this.currentFilter[item.index] = null;
            this.currentFilter["sort_" + item.index] = null;
        });
        this.responsiveBodyColumns.forEach(item => {
            this.currentFilter[item.index] = null;
            this.currentFilter["sort_" + item.index] = null;
        });
    }

    toggleVisibility(): void {
        this.isVisible = !this.isVisible;
    }

    emitFilter(index: number, filter: any): void {
        this.currentFilter[index] = filter;
        this.filterChanged.emit(this.currentFilter);
    }

    emitSort(index: number, state: number | null): void {
        this.currentFilter["sort_" + index] = state === null ? null : state == 1;
        this.filterChanged.emit(this.currentFilter);
    }
}
