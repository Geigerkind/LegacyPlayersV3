import {Component, EventEmitter, Input, OnChanges, Output} from "@angular/core";
import {DatePipe} from "@angular/common";
import {CharacterViewerDto} from "../../domain_value/character_viewer_dto";

@Component({
    selector: "CharacterItems",
    templateUrl: "./character_items.html",
    styleUrls: ["./character_items.scss"],
    providers: [DatePipe]
})
export class CharacterItemsComponent implements OnChanges {

    @Input() character: CharacterViewerDto;
    @Output() historyChanged: EventEmitter<number> = new EventEmitter<number>();

    constructor(
        private datePipe: DatePipe
    ) {
    }

    ngOnChanges(): void {
        this.character.history.sort((left, right) => {
            const leftNum = Number(left.label_key);
            const rightNum = Number(right.label_key);
            if (leftNum === rightNum) return 0;
            if (leftNum < rightNum) return 1;
            return -1;
        });
        this.character.history = this.character.history.map(history_moment => {
            const newHistoryMoment = history_moment;
            newHistoryMoment.label_key = this.datePipe.transform(new Date(Number(history_moment.label_key) * 1000), 'dd.MM.yy hh:mm a');
            return newHistoryMoment;
        });
    }

    emitHistory(history_id: number): void {
        if (history_id === this.character.history_id)
            return;
        this.historyChanged.emit(history_id);
    }

}
