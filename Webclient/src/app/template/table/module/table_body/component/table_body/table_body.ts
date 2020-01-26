import {Component, Input, OnChanges} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";
import {HeaderColumn} from "../../../table_header/domain_value/header_column";

@Component({
    selector: "TableBody",
    templateUrl: "./table_body.html",
    styleUrls: ["./table_body.scss"]
})
export class TableBodyComponent implements OnChanges {

    @Input() responsiveHeadColumns: Array<number>;
    @Input() isResponsiveMode: boolean;
    @Input() rows: Array<Array<BodyColumn>>;
    @Input() headColumns: HeaderColumn[];

    internalTypeRange: Map<number,string>[] = [];

    ngOnChanges(): void {
        this.headColumns.forEach(item => {
            if (this.internalTypeRange.length <= item.index)
                this.internalTypeRange.push(new Map<number, string>());
            if (item.type_range !== null)
                item.type_range.forEach(entry => this.internalTypeRange[item.index].set(entry.value, entry.labelKey));
        });
    }
}
