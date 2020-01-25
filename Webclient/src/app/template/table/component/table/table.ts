import {Component, EventEmitter, Input, Output} from "@angular/core";
import {WindowService} from "../../../../styling_service/window";
import {BodyColumn} from "../../module/table_body/domain_value/body_column";
import {HeaderColumn} from "../../module/table_header/domain_value/header_column";

@Component({
    selector: "Table",
    templateUrl: "./table.html",
    styleUrls: ["./table.scss"]
})
export class TableComponent {

    static readonly PAGE_SIZE: number = 10;

    @Output() filterOrPageChanged: EventEmitter<object> = new EventEmitter<object>();

    @Input() responsiveHeadColumns: number[] = [0,2];
    @Input() responsiveModeWidthInPx: number = 500;
    @Input() enableHeader: boolean = true;
    @Input() enableFooter: boolean = true;
    @Input() clientSide: boolean = true;
    @Input() headColumns: HeaderColumn[] = [
        {index: 0, labelKey: 'Test column 1', type: 0, type_range: undefined},
        {index: 1, labelKey: 'Test column 2', type: 1, type_range: undefined},
        {index: 2, labelKey: 'Test column 3', type: 2, type_range: undefined},
        {index: 3, labelKey: 'Test column 4', type: 3, type_range: undefined},
        {index: 4, labelKey: 'Test column 5', type: 3, type_range: ['Test0', 'Test1', 'Test2', 'Test3', 'Test4', 'Test5']}
    ];
    @Input()
    set bodyRows(rows: BodyColumn[][]) {
        this.bodyRowsData = rows;
        this.initFilter();
        this.setCurrentPageRows();
    }
    get bodyRows(): BodyColumn[][] {
        return this.bodyRowsData;
    }

    bodyRowsData: BodyColumn[][] = [];
    currentPageRows: BodyColumn[][] = [];
    isResponsiveMode: boolean = false;
    numItems: number = 0;

    private currentPageData: number = 0;
    private currentFilter: any = {};

    constructor(
        private windowService: WindowService
    ) {
        this.windowService.screenWidth$.subscribe((width) => this.isResponsiveMode = width <= this.responsiveModeWidthInPx);

        // DEBUG:
        // Generating test rows
        const now = new Date();
        for (let i = 0; i < 1000; ++i) {
            for (let j = 0; j < 5; ++j) {
                this.bodyRows.push([
                    {type: 0, content: 'Test ' + i + "-" + (i+j+2) + "-1"},
                    {type: 1, content: (6-j).toString()},
                    {type: 2, content: (new Date(now.getFullYear(), now.getMonth() + (i*j%10), now.getDate() - j).getTime()).toString()},
                    {type: 3, content: 'Test ' + i + "-" + j + "-4"},
                    {type: 3, content: 'Test'+j}
                ]);
            }
        }
    }

    set currentPage(page: number) {
        this.currentPageData = page - 1;
        this.currentFilter["page"] = this.currentPageData;
        if (this.clientSide)
            this.setCurrentPageRows();
        else
            this.filterOrPageChanged.emit(this.currentFilter);
    }
    get currentPage(): number {
        return this.currentPageData;
    }

    handleFilterChanged(filter: object): void {
        this.currentFilter = filter;
        this.currentFilter["page"] = this.currentPage;
        if (this.clientSide)
            this.setCurrentPageRows();
        else
            this.filterOrPageChanged.emit(this.currentFilter);
    }

    private initFilter(): void {
        this.headColumns.forEach(item => {
            this.currentPage["filter_" + item.index] = null;
            this.currentPage["sort_" + item.index] = null;
        })
    }

    private setCurrentPageRows(): void {
        const rows = this.applyFilter();
        this.numItems = rows.length;
        this.currentPageRows = rows
            .slice(this.currentPage * TableComponent.PAGE_SIZE, (this.currentPage+1) * TableComponent.PAGE_SIZE >= this.bodyRowsData.length ?
                                                                    this.bodyRowsData.length : (this.currentPage+1) * TableComponent.PAGE_SIZE);
    }

    private applyFilter(): BodyColumn[][] {
        if (!this.clientSide)
            return this.bodyRowsData;

        return this.bodyRowsData
            .filter(row => row.every((column, index) =>
                !this.currentFilter["filter_" + index] || this.currentFilter["filter_" + index].toString() === column.content || (
                    column.type === 0 && column.content.includes(this.currentFilter["filter_" + index])
                ) || (
                    column.type === 3 && this.headColumns[index].type_range[this.currentFilter["filter_" + index]-1] === column.content
                )))
            .sort((leftRow, rightRow) => {
               for (let index=0; index<leftRow.length; ++index) {
                   if (this.currentFilter["sort_" + index] === null)
                       continue;

                   const leftColumn: BodyColumn = leftRow[index];
                   const rightColumn: BodyColumn = rightRow[index];
                   const sorting: number = this.currentFilter["sort_" + index] === false ? 1 : -1;

                   if (leftColumn.type === 0 || leftColumn.type === 3) {
                       const result = leftColumn.content.localeCompare(rightColumn.content);
                       if (result === 0)
                           continue;
                       return result * sorting;
                   } else {
                       const leftNum = Number(leftColumn.content);
                       const rightNum = Number(rightColumn.content);
                       if (leftNum === rightNum)
                           continue;
                       return (leftNum > rightNum ? 1 : -1) * sorting;
                   }
               }
               return 0;
            });

    }
}
