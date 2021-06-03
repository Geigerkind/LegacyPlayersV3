import {Component} from "@angular/core";

@Component({
    selector: "RaidBrowser",
    templateUrl: "./raid_browser.html",
    styleUrls: ["./raid_browser.scss"]
})
export class RaidBrowserComponent {

    show_remove_widget: boolean = false;


    toggleRemoveWidget(): void {
        this.show_remove_widget = !this.show_remove_widget;
    }

    close_remove_widget(): void {
        this.show_remove_widget = false;
    }
}
