import {Component, OnInit} from "@angular/core";
import {SettingsService} from "../../../../service/settings";

@Component({
    selector: "NavigationBar",
    templateUrl: "./navigation_bar.html",
    styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBarComponent implements OnInit {
    /*
    itemsRaids = [
        ["/raids/", "NavigationBar.raids.title"],
        ["/raids/bosses/", "NavigationBar.raids.bosses"],
        ["/raids/ranking/", "NavigationBar.raids.ranking"],
        ["/raids/speed_kill/", "NavigationBar.raids.speed_kill"],
        ["/raids/speed_run/", "NavigationBar.raids.speed_run"],
        ["/raids/loot/", "NavigationBar.raids.loot"],
    ];
     */

    itemsArmory = [
        ["/armory/", "NavigationBar.armory.title"],
        /*
        ["/armory/items/", "NavigationBar.armory.items"],
        ["/armory/guilds/", "NavigationBar.armory.guilds"],
         */
    ];
/*
    itemsPvP = [
        ["/pvp/", "NavigationBar.pvp.title"],
        ["/pvp/standings/", "NavigationBar.pvp.standings"],
        ["/pvp/hk_life/", "NavigationBar.pvp.hk_life"],
        ["/pvp/arena/", "NavigationBar.pvp.arena"],
    ];

    itemsTools = [
        ["/tools/", "NavigationBar.tools.title"],
        ["/tools/talent_calc/", "NavigationBar.tools.talent_calc"],
        ["/tools/rp_calc/", "NavigationBar.tools.rp_calc"],
        ["/tools/char_design/", "NavigationBar.tools.char_design"],
    ];
*/

    itemsContribute = [
        ["/contribute/", "NavigationBar.contribute.title"]
    ];
/*
    itemsQueue = [
        ["/queue/", "NavigationBar.queue.title"]
    ];
*/
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
