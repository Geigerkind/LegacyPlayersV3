import {Component, EventEmitter, Output} from "@angular/core";

@Component({
    selector: "PopupAdd",
    templateUrl: "./popup_add.html",
    styleUrls: ["./popup_add.scss"]
})
export class PopupAddComponent {

    @Output() close_widget: EventEmitter<void> = new EventEmitter();

}
