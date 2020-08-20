import {Component, Input, OnChanges} from "@angular/core";
import {DeathOverviewRow} from "../../domain_value/death_overview_row";
import {UnitService} from "../../../../../../service/unit";
import {SpellService} from "../../../../../../service/spell";
import {DateService} from "../../../../../../../../service/date";

@Component({
    selector: "DeathsOverview",
    templateUrl: "./deaths_overview.html",
    styleUrls: ["./deaths_overview.scss"],
    providers: [
        SpellService,
        UnitService,
        DateService
    ]
})
export class DeathsOverviewComponent implements OnChanges {

    @Input() death_overview_rows: Array<DeathOverviewRow> = [];
    @Input() server_id: number;
    @Input() to_actor: boolean = true;
    @Input() bar_tooltips: Map<number, any>;

    constructor(
        public unitService: UnitService,
        public spellService: SpellService,
        public dateService: DateService
    ) {
    }

    ngOnChanges(): void {
        this.unitService.set_server_id(this.server_id);
        this.spellService.set_server_id(this.server_id);
    }

    get_tooltip(row_index: number): any {
        if (!this.bar_tooltips)
            return {};
        return this.bar_tooltips.get(row_index);
    }

    get routerLink(): string {
        if (this.to_actor)
            return "../event_log/to_actor";
        return "../event_log/by_actor";
    }
}
