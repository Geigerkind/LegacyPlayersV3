import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "HeaderRow",
    templateUrl: "./header_row.html",
    styleUrls: ["./header_row.scss"]
})
export class HeaderRowComponent implements OnInit {

    @Input() columns: HeaderColumn[];
    @Output() filterChanged: EventEmitter<string> = new EventEmitter<string>();

    currentFilter: object = {};

    ngOnInit(): void {
        this.columns.forEach(item => {
            this.currentFilter[item.filter_name] = {
                filter: null,
                sorting: null
            };
        });
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
