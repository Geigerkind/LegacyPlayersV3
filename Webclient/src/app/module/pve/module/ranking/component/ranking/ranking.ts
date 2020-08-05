import {Component, OnDestroy, OnInit} from "@angular/core";
import {RankingService} from "../../service/ranking";
import {RaidMeterSubject} from "../../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {SettingsService} from "../../../../../../service/settings";
import {Subscription} from "rxjs";
import {DataService} from "../../../../../../service/data";
import {take} from "rxjs/operators";

@Component({
    selector: "Ranking",
    templateUrl: "./ranking.html",
    styleUrls: ["./ranking.scss"],
    providers: [
        RankingService
    ]
})
export class RankingComponent implements OnInit, OnDestroy {

    private subscription_rankings: Subscription;
    private subscription_servers: Subscription;
    private subscription_hero_classes: Subscription;
    private subscription_boss_npcs: Subscription;

    bar_subjects: Map<number, RaidMeterSubject> = new Map();
    bar_tooltips: Map<number, any> = new Map();
    bars: Array<[number, number]> = [];

    modes_current_selection: number = 1;
    modes: Array<SelectOption> = [
        {value: 1, label_key: "Damage per second"},
        {value: 2, label_key: "Effective heal per second"},
        {value: 3, label_key: "Threat per second"},
    ];

    selections: Array<SelectOption> = [
        {value: 1, label_key: "Overall"}
    ];

    bosses_current_select: number = 1;
    bosses: Array<any> = [];

    classes: Array<any> = [];

    servers: Array<any> = [];

    constructor(
        private settingsService: SettingsService,
        private rankingService: RankingService,
        private dataService: DataService
    ) {
        this.subscription_rankings = this.rankingService.rankings.subscribe(entries => this.bars = entries);
        this.subscription_servers = this.dataService.servers.subscribe(servers => this.servers = servers.map(server => {
            return {id: server.id, label: server.name};
        }));
        this.subscription_hero_classes = this.dataService.hero_classes.subscribe(hero_classes => this.classes = hero_classes.map(hero_class => {
            return {id: hero_class.base.id, label: hero_class.localization};
        }));
        this.subscription_boss_npcs = this.dataService.boss_npcs.subscribe(boss_npcs => this.bosses = boss_npcs.map(boss_npc => {
            return {id: boss_npc.base.id, label: boss_npc.localization};
        }));
    }

    ngOnInit(): void {
        if (this.settingsService.check("pve_ranking"))
            this.modes_current_selection = this.settingsService.get("pve_ranking");
        this.modes_selection_changed(this.modes_current_selection);
    }

    ngOnDestroy(): void {
        this.subscription_rankings?.unsubscribe();
        this.subscription_hero_classes?.unsubscribe();
        this.subscription_boss_npcs?.unsubscribe();
        this.subscription_servers.unsubscribe();
    }

    bar_clicked(bar: [number, number]): void {

    }

    modes_selection_changed(selection: number): void {
        this.modes_current_selection = selection;
        this.settingsService.set("pve_ranking", this.modes_current_selection);
    }

    bosses_selection_changed(selection: number): void {
        this.bosses_current_select = selection;
    }

}
