import {Component} from "@angular/core";
import {BodyColumn} from "../../domain_value/body_column";

@Component({
    selector: "TableBody",
    templateUrl: "./table_body.html",
    styleUrls: ["./table_body.scss"]
})
export class TableBodyComponent {

    rows: BodyColumn[][] = [
        [
            { type: 0, content: 'Test 1' },
            { type: 0, content: 'Test 2' },
            { type: 0, content: 'Test 3' },
            { type: 0, content: 'Test 4' },
            { type: 0, content: 'Test 5' }
        ],
        [
            { type: 0, content: 'Test 1' },
            { type: 0, content: 'Test 2' },
            { type: 0, content: 'Test 3' },
            { type: 0, content: 'Test 4' },
            { type: 0, content: 'Test 5' }
        ]
    ];
}
