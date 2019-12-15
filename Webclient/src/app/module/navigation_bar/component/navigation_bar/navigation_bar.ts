import {Component, OnInit} from "@angular/core";
import {SettingsService} from "../../../../service/settings";

@Component({
    selector: "NavigationBar",
    templateUrl: "./navigation_bar.html",
    styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBarComponent implements OnInit {
    sampleItems = [
        ["url", "Entry 1"],
        ["url", "Entry 2"],
        ["url", "Entry 3"]
    ];

    accountItems: Array<Array<string>> = [
        ["/account/", "NavigationBar.account.title"],
        ["/logout/", "NavigationBar.account.logout"]
    ];

    loggedOutItems: Array<Array<string>> = [
        ["/login/", "NavigationBar.loggedOut.signIn"],
        ["/sign_up/", "NavigationBar.loggedOut.signUp"]
    ];

    show_item_list = false;
    loggedInState = false;

    constructor(private settingsService: SettingsService) {
        this.settingsService.subscribe("API_TOKEN", (api_token: any) => this.loggedInState = api_token && api_token.token.length > 0);
    }

    ngOnInit(): void {
        this.loggedInState = this.settingsService.check("API_TOKEN");
    }

    toggle(): void {
        this.show_item_list = !this.show_item_list;
    }

    handleClose(): void {
        this.show_item_list = false;
    }

}
