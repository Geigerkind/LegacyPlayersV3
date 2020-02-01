import {Component, Input, OnChanges} from "@angular/core";
import {DatePipe} from "@angular/common";

@Component({
    selector: "CharacterItems",
    templateUrl: "./character_items.html",
    styleUrls: ["./character_items.scss"],
    providers: [DatePipe]
})
export class CharacterItemsComponent implements OnChanges {

    @Input() character: any;

    constructor(
        private datePipe: DatePipe
    ) {
    }

    ngOnChanges(): void {
        this.character.history = this.character.history.map(history_moment => {
            const newHistoryMoment = history_moment;
            newHistoryMoment.label_key = this.datePipe.transform(new Date(history_moment.label_key*1000), 'dd.MM.yy hh:mm a');
            return newHistoryMoment;
        })
    }

}
