import {Component, OnDestroy, OnInit} from "@angular/core";
import {LootService} from "../../service/loot";
import {Subscription} from "rxjs";
import {Loot} from "../../domain_value/loot";

@Component({
    selector: "RaidLoot",
    templateUrl: "./raid_loot.html",
    styleUrls: ["./raid_loot.scss"],
    providers: [
        LootService
    ]
})
export class RaidLootComponent implements OnDestroy {

    private subscription: Subscription;

    loot: Array<Loot> = [];

    show_insignificant: boolean = false;
    toggle_decisions = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];

    constructor(
        private lootService: LootService
    ) {
        this.subscription = this.lootService.loot.subscribe(loot => this.loot = loot);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }
}
