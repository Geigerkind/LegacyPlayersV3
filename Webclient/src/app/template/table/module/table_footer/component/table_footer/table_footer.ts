import {Component, EventEmitter, Input, Output} from "@angular/core";

@Component({
    selector: "TableFooter",
    templateUrl: "./table_footer.html",
    styleUrls: ["./table_footer.scss"]
})
export class TableFooterComponent {

    leftText: string = '100 total';
    @Input() numPages: number;

    @Output() pageChanged: EventEmitter<number> = new EventEmitter<number>();

    bubblePageChanged(pageNumber: number) {
        this.pageChanged.emit(pageNumber);
    }
}
