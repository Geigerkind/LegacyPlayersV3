import {Component, Input} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "ResponsiveBodyRow",
    templateUrl: "./responsive_body_row.html",
    styleUrls: ["./responsive_body_row.scss"]
})
export class ResponsiveBodyRowComponent {
    @Input() responsiveHeaderColumns: Array<BodyColumn>;
    @Input() responsiveBodyColumns: Array<BodyColumn>;

    isVisible: boolean = false;

    toggleVisibility(): void {
        this.isVisible = !this.isVisible;
    }
}
