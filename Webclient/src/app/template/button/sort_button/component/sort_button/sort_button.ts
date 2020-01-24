import {Component, EventEmitter, Output} from "@angular/core";

@Component({
    selector: "SortButton",
    templateUrl: "./sort_button.html",
    styleUrls: ["./sort_button.scss"]
})
export class SortButtonComponent {
    @Output() state: EventEmitter<number> = new EventEmitter<number>();
    currentState: number = 0;

    nextState(): void {
        this.currentState = (this.currentState + 1) % 3;
        this.state.emit(this.currentState);
    }
}
