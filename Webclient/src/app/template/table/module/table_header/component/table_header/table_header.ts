import {Component} from "@angular/core";
import {HeaderColumn} from "../../domain_value/header_column";

@Component({
    selector: "TableHeader",
    templateUrl: "./table_header.html",
    styleUrls: ["./table_header.scss"]
})
export class TableHeaderComponent {

    head_columns: HeaderColumn[] = [
        {labelKey: 'Test column 1', type: 0, type_range: undefined},
        {labelKey: 'Test column 2', type: 1, type_range: undefined},
        {labelKey: 'Test column 3', type: 2, type_range: undefined},
        {labelKey: 'Test column 4', type: 3, type_range: undefined},
        {labelKey: 'Test column 5', type: 3, type_range: ['Test1', 'Test2', 'Test3']}
    ];
}
