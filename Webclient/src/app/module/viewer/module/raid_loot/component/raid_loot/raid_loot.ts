import {Component} from "@angular/core";

@Component({
    selector: "RaidLoot",
    templateUrl: "./raid_loot.html",
    styleUrls: ["./raid_loot.scss"]
})
export class RaidLootComponent {

    loot = [
        {
            boss_name: "The Prophet Skeram",
            items: [
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                },
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                },
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                },
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                },
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                }
            ]
        },
        {
            boss_name: "Edwin Van Cleef",
            items: [
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                }
            ]
        },
        {
            boss_name: "Cookie",
            items: [
                {
                    item_id: 1234,
                    icon_path: "/assets/wow_icon/inv_kilt_cloth_02.jpg",
                    quality: 5,
                    character_id: 11,
                    amount: 32
                }
            ]
        }
    ];

    toggle_decisions = [ false, false, false ];

}
