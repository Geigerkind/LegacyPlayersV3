import {Component, Input} from "@angular/core";
import {EventLogEntry} from "../../../../../viewer/module/raid_event_log/domain_value/event_log_entry";
import {DateService} from "../../../../../../service/date";

@Component({
    selector: "ViewerEventLogTooltip",
    templateUrl: "./viewer_event_log_tooltip.html",
    styleUrls: ["./viewer_event_log_tooltip.scss"]
})
export class ViewerEventLogTooltipComponent {

    @Input() payload: Array<EventLogEntry>;

    constructor(
        public dateService: DateService
    ) {
    }

}
