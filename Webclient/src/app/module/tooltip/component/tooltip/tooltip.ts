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

            if (args.type === 1) {
                this.tooltipService.loadCharacterTooltip(args.character_id, result => {
                    this.tooltipType = args.type;
                    this.tooltipPayload = result;
                    console.log("RESULT", result);
                });
            }
        });
    }

}
