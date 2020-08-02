import {Component} from "@angular/core";
import {TooltipControllerService} from "../../../../service/tooltip_controller";
import {TooltipService} from "../../service/tooltip";

@Component({
    selector: "Tooltip",
    templateUrl: "./tooltip.html",
    styleUrls: ["./tooltip.scss"]
})
export class TooltipComponent {

    iconPath: string;
    tooltipType: number;
    tooltipPayload: any;

    constructor(
        private tooltipControllerService: TooltipControllerService,
        private tooltipService: TooltipService
    ) {
        this.tooltipControllerService.subscribe(args => {
            this.tooltipType = undefined;
            this.tooltipPayload = undefined;
            this.iconPath = undefined;

            if (args.type === 1) {
                this.tooltipService.loadCharacterTooltip(args.character_id, result => {
                    this.tooltipType = args.type;
                    this.tooltipPayload = result;
                    this.iconPath = "/assets/wow_hero_classes/c" + result.hero_class_id + ".png";
                });
            } else if (args.type === 2) {
                this.tooltipService.loadCharacterItemTooltip(args.history_id, args.item_id, result => {
                    this.tooltipType = args.type;
                    this.tooltipPayload = result;
                    this.iconPath = "/assets/wow_icon/" + result.icon + ".jpg";
                });
            } else if (args.type === 3) {
                this.tooltipService.loadGuildTooltip(args.guild_id, result => {
                    this.tooltipType = args.type;
                    this.tooltipPayload = result;
                });
            } else if (args.type === 4) {
                this.tooltipService.loadItemTooltip(args.expansion_id, args.item_id, result => {
                    this.tooltipType = args.type;
                    this.tooltipPayload = result;
                    this.iconPath = "/assets/wow_icon/" + result.icon + ".jpg";
                });
            } else if (args.type === 5) {
                this.tooltipType = args.type;
                this.tooltipPayload = args.payload;
            }
        });
    }

}
