import {Component, Input} from "@angular/core";

@Component({
    selector: "HeaderTd",
    templateUrl: "./header_td.html",
    styleUrls: ["./header_td.scss"]
})
export class HeaderTdComponent {
    @Input() labelKey: string;

    showFilter: boolean = false;
    filterValue: string = '';

    columnClicked(): void {
        this.showFilter = true;
    }

    leaveFocus(): void {
        if (this.filterValue === '') {
            this.showFilter = false;
        }
    }

}
