import {Component} from "@angular/core";

@Component({
    selector: "RaidComposition",
    templateUrl: "./raid_composition.html",
    styleUrls: ["./raid_composition.scss"]
})
export class RaidCompositionComponent {

    temp_classes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    show_tanks: boolean = false;
    show_healers: boolean = false;
    show_dps: boolean = false;

}
