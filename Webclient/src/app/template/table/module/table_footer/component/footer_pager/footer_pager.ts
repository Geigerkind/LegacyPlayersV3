import {Component} from "@angular/core";

@Component({
    selector: "FooterPager",
    templateUrl: "./footer_pager.html",
    styleUrls: ["./footer_pager.scss"]
})
export class FooterPagerComponent {

    private static readonly PAGE_SIZE: number = 10;
    currentPage: number = 1;
    numPages: number = 10;

    goToFirstPage(): void {
        this.currentPage = 1;
    }

    goToLastPage(): void {
        this.currentPage = this.numPages;
    }

    goToNextPage(): void {
        if (this.currentPage + 1 <= this.numPages)
            ++this.currentPage;
    }

    goToPreviousPage(): void {
        if (this.currentPage - 1 >= 1)
            --this.currentPage;
    }
}
