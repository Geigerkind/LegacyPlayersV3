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
        return socket.slots.every((slot) => slot.item && slot.flag === slot.item.flag);
    }

    isEffectActive(items: any, threshold: number): boolean {
        return items.filter(item => item.active).length >= threshold;
    }

    getFilteredSetItems(): any {
        return this.payload.item_set.set_items
            .filter(item => item.active || item.item_level === this.payload.item_level)
            .sort((left, right) => {
                if (left.active === right.active) return 0;
                if (left.active !== right.active) return 1;
                return -1;
            })
            .reduce((acc, item) => {
                if (item.active || !acc.find(inner_item => inner_item.inventory_type === item.inventory_type))
                    acc.push(item);
                return acc;
            }, [])
            .sort((left, right) => {
                if (left.inventory_type === right.inventory_type) return 0;
                if (left.inventory_type > right.inventory_type) return 1;
                return -1;
            });
    }

}
