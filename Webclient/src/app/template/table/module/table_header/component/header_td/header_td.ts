import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";
import {SelectOption} from "../../../../../input/select_input/domain_value/select_option";

@Component({
    selector: "HeaderTd",
    templateUrl: "./header_td.html",
    styleUrls: ["./header_td.scss"]
})
export class HeaderTdComponent implements OnInit {
    @Input() specification: HeaderColumn;
    @Output() filterChanged: EventEmitter<any> = new EventEmitter<any>();
    @Output() sortChanged: EventEmitter<number | null> = new EventEmitter<number | null>();

    showFilter: boolean = false;
    filterValueData: any;
    filterRange: SelectOption[];

    set filterValue(value: any) {
        this.filterValueData = value;
        if (!this.isFilterDefault()) {
            if (this.specification.type === 2)
                this.filterChanged.emit(new Date(this.filterValueData).getTime());
            else
                this.filterChanged.emit(this.filterValueData);
        } else {
            this.filterChanged.emit(null);
        }
    }
    get filterValue(): any {
        return this.filterValueData;
    }

    ngOnInit(): void {
        this.filterValue = this.defaultFilterValue();
        if (this.specification.type === 3) {
            this.filterRange = [{value: 0, labelKey: this.specification.labelKey}];
            if (this.specification.type_range)
                this.specification.type_range.forEach((item, i) => this.filterRange.push({value: i+1, labelKey: item}));
        }
    }

    defaultFilterValue(): any {
        if (this.specification.type == 2)
            return null;
        else if (this.specification.type > 1)
            return 0;
        return '';
    }

    columnClicked(): void {
        this.showFilter = true;
    }

    leaveFocus(): void {
        if (this.isFilterDefault())
            this.showFilter = false;
    }

    sortStateChanged(state: number): void {
        this.sortChanged.emit(state === 0 ? null : state);
    }

    private isFilterDefault(): boolean {
        return this.filterValue === this.defaultFilterValue() || (this.specification.type === 2 && this.filterValue.getTime && !this.filterValue.getTime());
    }

}
