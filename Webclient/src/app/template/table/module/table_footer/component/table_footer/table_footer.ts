import {Component} from "@angular/core";

@Component({
    selector: "TableFooter",
    templateUrl: "./table_footer.html",
    styleUrls: ["./table_footer.scss"]
})
export class TableFooterComponent {

    leftText: string = '1 to 10 of 100 total';
    rightText: string = 'Test';

}
