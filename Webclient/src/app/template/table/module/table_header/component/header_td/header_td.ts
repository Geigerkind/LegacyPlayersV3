import {Component, Input, OnInit} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";
import {SelectOption} from "../../../../../input/select_input/domain_value/select_option";

@Component({
    selector: "HeaderTd",
    templateUrl: "./header_td.html",
    styleUrls: ["./header_td.scss"]
})
export class HeaderTdComponent implements OnInit {
    @Input() specification: HeaderColumn;

    showFilter: boolean = false;
    filterValue: any;
    filterRange: SelectOption[];

    ngOnInit(): void {
        this.filterValue = this.defaultFilterValue();
        if (this.specification.type === 3) {
            this.filterRange = [{value: 0, labelKey: this.specification.labelKey}];
            if (this.specification.type_range)
                this.specification.type_range.forEach((item, i) => this.filterRange.push({value: i+1, labelKey: item}));
        }
    }

    defaultFilterValue(): any {
        if (this.specification.type == 2) {
            const now = new Date();
            return new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, 0);
        } else if (this.specification.type > 0)
            return 0;
        return '';
    }

    columnClicked(): void {
        this.showFilter = true;
    }

    leaveFocus(): void {
        if ((this.specification.type == 2 && this.filterValue.getTime() === this.defaultFilterValue().getTime()) || this.filterValue == this.defaultFilterValue()) {
            this.showFilter = false;
        }
    }

}
