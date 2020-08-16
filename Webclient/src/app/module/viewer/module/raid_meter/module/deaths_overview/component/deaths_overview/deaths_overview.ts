import {Component, Input, OnChanges} from "@angular/core";
import {DeathOverviewRow} from "../../domain_value/death_overview_row";
import {UnitService} from "../../../../../../service/unit";
import {SpellService} from "../../../../../../service/spell";
import {DateService} from "../../../../../../../../service/date";
import {EventLogService} from "../../../../../raid_event_log/service/event_log";
import {map} from "rxjs/operators";

@Component({
    selector: "DeathsOverview",
    templateUrl: "./deaths_overview.html",
    styleUrls: ["./deaths_overview.scss"],
    providers: [
        SpellService,
        UnitService,
        DateService,
        // Tooltip
        EventLogService
    ]
})
export class DeathsOverviewComponent implements OnChanges {

    @Input() death_overview_rows: Array<DeathOverviewRow> = [];
    @Input() server_id: number;

    constructor(
        public unitService: UnitService,
        public spellService: SpellService,
        public dateService: DateService,
        private eventLogService: EventLogService
    ) {
        this.eventLogService.set_actor(true);
    }

    ngOnChanges(): void {
        this.unitService.set_server_id(this.server_id);
        this.spellService.set_server_id(this.server_id);
    }

    get_tooltip(row: DeathOverviewRow): any {
        return {
            type: 8,
            payload: this.eventLogService.event_log_entries
                .pipe(map(events => events.filter(event => event.timestamp <= row.timestamp).slice(0, 10)))
        };
    }
}
