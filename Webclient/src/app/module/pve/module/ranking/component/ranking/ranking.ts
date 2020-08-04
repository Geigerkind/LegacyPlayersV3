import {Component, OnDestroy, OnInit} from "@angular/core";
import {RankingService} from "../../service/ranking";
import {RaidMeterSubject} from "../../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {SettingsService} from "../../../../../../service/settings";
import {Subscription} from "rxjs";

@Component({
    selector: "Ranking",
    templateUrl: "./ranking.html",
    styleUrls: ["./ranking.scss"],
    providers: [
        RankingService
    ]
})
export class RankingComponent implements OnInit, OnDestroy {

    private subscription: Subscription;

    bar_subjects: Map<number, RaidMeterSubject> = new Map();
    bar_tooltips: Map<number, any> = new Map();
    bars: Array<[number, number]> = [];

    current_selection: number = 1;
    options: Array<SelectOption> = [
        {value: 1, label_key: "Damage per second"},
        {value: 2, label_key: "Effective heal per second"},
        {value: 3, label_key: "Threat per second"},
    ];

    constructor(
        private settingsService: SettingsService,
        private rankingService: RankingService
    ) {
        this.subscription = this.rankingService.rankings.subscribe(entries => this.bars = entries);
    }

    ngOnInit(): void {
        if (this.settingsService.check("pve_ranking"))
            this.current_selection = this.settingsService.get("pve_ranking");
        this.selection_changed(this.current_selection);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    bar_clicked(bar: [number, number]): void {

    }

    selection_changed(selection: number): void {
        this.current_selection = selection;
        this.settingsService.set("pve_ranking", this.current_selection);
    }

}
