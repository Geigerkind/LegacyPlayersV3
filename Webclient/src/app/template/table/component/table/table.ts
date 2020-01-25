import {Component, Input} from "@angular/core";
import {WindowService} from "../../../../styling_service/window";
import {BodyColumn} from "../../module/table_body/domain_value/body_column";
import {HeaderColumn} from "../../module/table_header/domain_value/header_column";

@Component({
    selector: "Table",
    templateUrl: "./table.html",
    styleUrls: ["./table.scss"]
})
export class TableComponent {

    @Input() responsiveHeadColumns: number[] = [0,2];
    @Input() responsiveModeWidthInPx: number = 500;
    @Input() enableHeader: boolean = true;
    @Input() enableFooter: boolean = true;
    @Input() filterClientSide: boolean = true;
    @Input() pageClientSide: boolean = true;
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
    @Input() headColumns: HeaderColumn[] = [
        {labelKey: 'Test column 1', type: 0, type_range: undefined},
        {labelKey: 'Test column 2', type: 1, type_range: undefined},
        {labelKey: 'Test column 3', type: 2, type_range: undefined},
        {labelKey: 'Test column 4', type: 3, type_range: undefined},
        {labelKey: 'Test column 5', type: 3, type_range: ['Test1', 'Test2', 'Test3']}
    ];

    isResponsiveMode: boolean = false;

    constructor(
        private windowService: WindowService
    ) {
        this.windowService.screenWidth$.subscribe((width) => this.isResponsiveMode = width <= this.responsiveModeWidthInPx);
    }
}
