import {Component, OnDestroy} from "@angular/core";
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
    toggle_decisions = [];


    constructor(
        private lootService: LootService
    ) {
        this.subscription = this.lootService.loot.subscribe(loot => {
            this.toggle_decisions = Array(loot.length).fill(false);
            this.loot = loot;
        });
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }
}
