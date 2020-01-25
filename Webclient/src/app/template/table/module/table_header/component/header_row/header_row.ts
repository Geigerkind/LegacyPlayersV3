import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "HeaderRow",
    templateUrl: "./header_row.html",
    styleUrls: ["./header_row.scss"]
})
export class HeaderRowComponent implements OnInit {

    @Input() columns: HeaderColumn[];
    @Output() filterChanged: EventEmitter<object> = new EventEmitter<object>();

    currentFilter: object = {};

    ngOnInit(): void {
        this.columns.forEach(item => {
            this.currentFilter[item.index] = null;
            this.currentFilter["sort_" + item.index] = null;
        });
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
