import {Component, Input, OnChanges} from "@angular/core";
import {HeaderColumn} from "../../../table_header/domain_value/header_column";
import {BodyRow} from "../../domain_value/body_row";

@Component({
    selector: "TableBody",
    templateUrl: "./table_body.html",
    styleUrls: ["./table_body.scss"]
})
export class TableBodyComponent implements OnChanges {

    @Input() responsiveHeadColumns: Array<number>;
    @Input() isResponsiveMode: boolean;
    @Input() rows: Array<BodyRow>;
    @Input() headColumns: Array<HeaderColumn>;

    internalTypeRange: Array<Map<number, string>> = [];

    ngOnChanges(): void {
        this.updateInternalTypeRange();
    }

    updateInternalTypeRange(): void {
        this.headColumns.forEach(item => {
            if (this.internalTypeRange.length <= item.index)
                this.internalTypeRange.push(new Map<number, string>());
            if (item.type === 3) {
                item.type_range.forEach(entry => this.internalTypeRange[item.index].set(entry.value, entry.labelKey));

                // Horrible hack, but it circumvents Angular's problem with property bindings on objects
                if (this.internalTypeRange[item.index].size === 1) {
                    setTimeout(() => this.updateInternalTypeRange(), 50);
                }
            }
        });
    }
}
