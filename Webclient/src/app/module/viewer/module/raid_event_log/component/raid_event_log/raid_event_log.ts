import {Component, OnDestroy} from "@angular/core";
import {EventLogService} from "../../service/event_log";
import {EventLogEntry} from "../../domain_value/event_log_entry";
import {Observable, Subscription} from "rxjs";
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
export class RaidEventLogComponent implements OnDestroy {

    private subscription: Subscription;

    event_log_entries: Observable<Array<EventLogEntry>>;
    private current_offset: number = 0;
    private event_log_entries_length: number = 0;

    constructor(
        private eventLogService: EventLogService,
        public dateService: DateService,
        private activatedRouteService: ActivatedRoute
    ) {
        this.event_log_entries = this.eventLogService.event_log_entries;
        this.activatedRouteService.firstChild?.paramMap.subscribe(params => {
            this.current_offset = 0;
            this.event_log_entries_length = 0;
            this.eventLogService.offset_changed.next(this.current_offset);
            this.eventLogService.set_actor(params.get("actor") === "to_actor");
        });
        this.subscription = this.event_log_entries.subscribe(entries => this.event_log_entries_length = entries.length);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    // TODO: Mobile?
    scrolled(scroll_event: any): void {
        if (scroll_event.deltaY > 0) {
            if (this.event_log_entries_length === 19)
                ++this.current_offset;
        } else {
            --this.current_offset;
            if (this.current_offset < 0)
                this.current_offset = 0;
        }
        this.eventLogService.offset_changed.next(this.current_offset);
    }

}
