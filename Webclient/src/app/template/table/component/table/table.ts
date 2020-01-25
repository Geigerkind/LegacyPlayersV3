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

    private static readonly PAGE_SIZE: number = 10;

    @Input() responsiveHeadColumns: number[] = [0,2];
    @Input() responsiveModeWidthInPx: number = 500;
    @Input() enableHeader: boolean = true;
    @Input() enableFooter: boolean = true;
    @Input() filterClientSide: boolean = true;
    @Input() pageClientSide: boolean = true;
    @Input() bodyRows: BodyColumn[][] = [];
    @Input() headColumns: HeaderColumn[] = [
        {labelKey: 'Test column 1', type: 0, type_range: undefined},
        {labelKey: 'Test column 2', type: 1, type_range: undefined},
        {labelKey: 'Test column 3', type: 2, type_range: undefined},
        {labelKey: 'Test column 4', type: 3, type_range: undefined},
        {labelKey: 'Test column 5', type: 3, type_range: ['Test1', 'Test2', 'Test3']}
    ];

    currentPageData: number = 0;
    isResponsiveMode: boolean = false;

    constructor(
        private windowService: WindowService
    ) {
        this.windowService.screenWidth$.subscribe((width) => this.isResponsiveMode = width <= this.responsiveModeWidthInPx);

        // DEBUG:
        // Generating test rows
        for (let i = 0; i < 1000; ++i) {
            for (let j = 0; j < 5; ++j) {
                this.bodyRows.push([
                    {type: 0, content: 'Test ' + i + "-" + j + "-1"},
                    {type: 0, content: 'Test ' + i + "-" + j + "-2"},
                    {type: 0, content: 'Test ' + i + "-" + j + "-3"},
                    {type: 0, content: 'Test ' + i + "-" + j + "-4"},
                    {type: 0, content: 'Test ' + i + "-" + j + "-5"}
                ]);
            }
        }
    }

    set currentPage(page: number) {
        this.currentPageData = page - 1;
    }
    get currentPage(): number {
        return this.currentPageData;
    }

    getCurrentPage(): BodyColumn[][] {
        return this.bodyRows.slice(this.currentPage * TableComponent.PAGE_SIZE, (this.currentPage+1) * TableComponent.PAGE_SIZE >= this.bodyRows.length ? this.bodyRows.length : (this.currentPage+1) * TableComponent.PAGE_SIZE);
    }

    getNumPages(): number {
        return Math.ceil(this.bodyRows.length / TableComponent.PAGE_SIZE);
    }
}
