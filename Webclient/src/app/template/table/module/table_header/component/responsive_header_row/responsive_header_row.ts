import {Component, Input} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "ResponsiveHeaderRow",
    templateUrl: "./responsive_header_row.html",
    styleUrls: ["./responsive_header_row.scss"]
})
export class ResponsiveHeaderRowComponent {

    @Input() responsiveHeaderColumns: HeaderColumn[];
    @Input() responsiveBodyColumns: HeaderColumn[];

    isVisible: boolean = false;

    toggleVisibility(): void {
        this.isVisible = !this.isVisible;
    }

}
