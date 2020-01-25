import {Component, Input} from "@angular/core";
import {WindowService} from "../../../../styling_service/window";
import {BodyColumn} from "../../module/table_body/domain_value/body_column";

@Component({
    selector: "Table",
    templateUrl: "./table.html",
    styleUrls: ["./table.scss"]
})
export class TableComponent {

    @Input() responsiveHeadColumns: number[] = [0];
    @Input() responsiveModeWidthInPx: number = 500;
    @Input() enableHeader: boolean = true;
    @Input() enableFooter: boolean = true;
    @Input() bodyRows: BodyColumn[][] = [
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

    isResponsiveMode: boolean = false;

    constructor(
        private windowService: WindowService
    ) {
        this.windowService.screenWidth$.subscribe((width) => this.isResponsiveMode = width <= this.responsiveModeWidthInPx);
    }
}
