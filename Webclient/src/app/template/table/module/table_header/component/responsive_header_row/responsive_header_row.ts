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
    @Output() filterChanged: EventEmitter<string> = new EventEmitter<string>();

    isVisible: boolean = false;
    currentFilter: object = {};

    ngOnInit(): void {
        this.responsiveHeaderColumns.forEach(item => {
            this.currentFilter[item.filter_name] = {
                filter: null,
                sorting: null
            };
        });
        this.responsiveBodyColumns.forEach(item => {
            this.currentFilter[item.filter_name] = {
                filter: null,
                sorting: null
            };
        });
    }

    toggleVisibility(): void {
        this.isVisible = !this.isVisible;
    }

    emitFilter(filter_name: string, filter: any): void {
        if (this.currentFilter[filter_name]["filter"] !== filter) {
            this.currentFilter[filter_name]["filter"] = filter;
            this.filterChanged.emit(JSON.stringify(this.currentFilter));
        }
    }

    emitSort(filter_name: string, state: number | null): void {
        const newStateValue = state === null ? null : state === 1;
        if (this.currentFilter[filter_name]["sorting"] !== newStateValue) {
            this.currentFilter[filter_name]["sorting"] = newStateValue;
            this.filterChanged.emit(JSON.stringify(this.currentFilter));
        }
    }
}
