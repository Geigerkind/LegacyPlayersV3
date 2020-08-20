import {Component, Input, OnChanges} from "@angular/core";
import {UnitService} from "../../../../../../service/unit";
import {SpellService} from "../../../../../../service/spell";
import {DateService} from "../../../../../../../../service/date";
import {UnAuraOverviewRow} from "../../domain_value/un_aura_overview_row";

@Component({
    selector: "UnAuraOverview",
    templateUrl: "./un_aura_overview.html",
    styleUrls: ["./un_aura_overview.scss"],
    providers: [
        SpellService,
        UnitService,
        DateService
    ]
})
export class UnAuraOverviewComponent implements OnChanges {

    @Input() un_aura_overview_rows: Array<UnAuraOverviewRow> = [];
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
