import {Component, EventEmitter, Input, Output} from "@angular/core";
import {Preset} from "../../../../domain_value/preset";

@Component({
    selector: "PopupAdd",
    templateUrl: "./popup_add.html",
    styleUrls: ["./popup_add.scss"]
})
export class PopupAddComponent {

    @Input() presets: Array<Preset>;
    @Output() close_widget: EventEmitter<void> = new EventEmitter();

}
