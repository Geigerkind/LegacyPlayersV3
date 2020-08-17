import {Component, Input, OnChanges} from "@angular/core";
import {UnitService} from "../../../../../../service/unit";
import {SpellService} from "../../../../../../service/spell";
import {DateService} from "../../../../../../../../service/date";
import {EventLogService} from "../../../../../raid_event_log/service/event_log";
import {map} from "rxjs/operators";
import {UnAuraOverviewRow} from "../../domain_value/un_aura_overview_row";

@Component({
    selector: "UnAuraOverview",
    templateUrl: "./un_aura_overview.html",
    styleUrls: ["./un_aura_overview.scss"],
    providers: [
        SpellService,
        UnitService,
        DateService,
        // Tooltip
        EventLogService
    ]
})
export class UnAuraOverviewComponent implements OnChanges {

    @Input() un_aura_overview_rows: Array<UnAuraOverviewRow> = [];
    @Input() server_id: number;
    @Input() to_actor: boolean = true;

    constructor(
        public unitService: UnitService,
        public spellService: SpellService,
        public dateService: DateService,
        private eventLogService: EventLogService
    ) {
    }

    ngOnChanges(): void {
        this.unitService.set_server_id(this.server_id);
        this.spellService.set_server_id(this.server_id);
        this.eventLogService.set_actor(this.to_actor);
    }

    get_tooltip(row: UnAuraOverviewRow): any {
        return {
            type: 8,
            payload: this.eventLogService.event_log_entries
                .pipe(map(events => events.filter(event => event.timestamp <= row.timestamp).slice(0, 10)))
        };
    }

    get routerLink(): string {
        if (this.to_actor)
            return "../event_log/to_actor";
        return "../event_log/by_actor";
    }
}
