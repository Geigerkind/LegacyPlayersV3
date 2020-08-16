import {Component} from "@angular/core";
import {EventLogService} from "../../service/event_log";
import {EventLogEntry} from "../../domain_value/event_log_entry";
import {Observable} from "rxjs";
import {DateService} from "../../../../../../service/date";
import {ActivatedRoute} from "@angular/router";

@Component({
    selector: "RaidEventLog",
    templateUrl: "./raid_event_log.html",
    styleUrls: ["./raid_event_log.scss"],
    providers: [
        EventLogService
    ]
})
export class RaidEventLogComponent {

    event_log_entries: Observable<Array<EventLogEntry>>;

    constructor(
        private eventLogService: EventLogService,
        public dateService: DateService,
        private activatedRouteService: ActivatedRoute
    ) {
        this.event_log_entries = this.eventLogService.event_log_entries;
        this.activatedRouteService.firstChild?.paramMap.subscribe(params => this.eventLogService.set_actor(params.get("actor") === "to_actor"));
    }

}
