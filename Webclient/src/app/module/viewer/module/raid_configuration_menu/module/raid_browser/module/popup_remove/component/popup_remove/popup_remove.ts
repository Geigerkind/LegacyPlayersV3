import {Component, EventEmitter, Output} from "@angular/core";

@Component({
    selector: "PopupRemove",
    templateUrl: "./popup_remove.html",
    styleUrls: ["./popup_remove.scss"]
})
export class PopupRemoveComponent {

    @Output() close_widget: EventEmitter<void> = new EventEmitter();

}
