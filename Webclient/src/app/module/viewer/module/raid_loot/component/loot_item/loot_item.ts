import {Component, Input} from "@angular/core";

@Component({
    selector: "LootItem",
    templateUrl: "./loot_item.html",
    styleUrls: ["./loot_item.scss"]
})
export class LootItemComponent {

    @Input() item_id: number;
    @Input() icon_path: string;
    @Input() quality: number;
    @Input() character_id: number;
    @Input() amount: number;

}
