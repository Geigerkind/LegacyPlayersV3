import {Component, Input} from "@angular/core";
import {ItemTooltip} from "../../domain_value/item_tooltip";

@Component({
    selector: "ItemTooltip",
    templateUrl: "./item_tooltip.html",
    styleUrls: ["./item_tooltip.scss"]
})
export class ItemTooltipComponent {

    @Input() payload: ItemTooltip;

    validSocketBonus(socket: any): boolean {
        return socket.slots.every((slot) => slot.item && slot.flag === slot.item.flag)
    }

    isEffectActive(items: any, threshold: number): boolean {
        return items.filter(item => item.active).length >= threshold;
    }

}
