import {Component, Input} from "@angular/core";

@Component({
    selector: "YesNoSwitch",
    templateUrl: "./yes_no_switch.html",
    styleUrls: ["./yes_no_switch.scss"]
})
export class YesNoSwitchComponent {

    @Input() state: boolean;

}
