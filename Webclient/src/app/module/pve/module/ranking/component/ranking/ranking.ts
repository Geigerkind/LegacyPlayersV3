import {Component, OnDestroy, OnInit} from "@angular/core";
import {RankingService} from "../../service/ranking";
import {RaidMeterSubject} from "../../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {SettingsService} from "../../../../../../service/settings";
import {of, Subscription} from "rxjs";
import {DataService} from "../../../../../../service/data";
import {TinyUrlService} from "../../../../../tiny_url/service/tiny_url";
import {TinyUrl} from "../../../../../tiny_url/domain_value/tiny_url";
import {RankingUrl} from "../../../../../tiny_url/domain_value/ranking_url";

@Component({
    selector: "Ranking",
    templateUrl: "./ranking.html",
    styleUrls: ["./ranking.scss"],
    providers: [
        RankingService,
        TinyUrlService
    ]
})
export class RankingComponent implements OnInit, OnDestroy {

    private subscription_rankings: Subscription;
    private subscription_servers: Subscription;
    private subscription_hero_classes: Subscription;
    private subscription_encounters: Subscription;

    bar_subjects: Map<number, RaidMeterSubject> = new Map();
    bar_tooltips: Map<number, any> = new Map();
    bars: Array<[number, number]> = [];

    modes_current_selection: number = 1;
    modes: Array<SelectOption> = [
        {value: 1, label_key: "Damage per second"},
        {value: 2, label_key: "Effective heal per second"},
        {value: 3, label_key: "Threat per second"},
    ];

    selections_current_selection: number = 1;
    selections: Array<SelectOption> = [
        {value: 1, label_key: "Overall"}
    ];

    encounters_selected_items: Array<any> = [];
    encounters: Array<any> = [];

    classes_selected_items: Array<any> = [];
    classes: Array<any> = [];

    servers_selected_items: Array<any> = [];
    servers: Array<any> = [];

    private finished_loading: [boolean, boolean, boolean] = [false, false, false];

    constructor(
        private settingsService: SettingsService,
        private rankingService: RankingService,
        private dataService: DataService,
        private tinyUrlService: TinyUrlService
    ) {
        this.subscription_rankings = this.rankingService.rankings.subscribe(entries => {
            for (const row of entries) {
                this.bar_subjects.set(row.character_id, {
                    color_class: of("hero_class_bg_" + row.character_meta.hero_class_id.toString()),
                    icon: of("/assets/wow_hero_classes/c" + row.character_meta.hero_class_id.toString() + ".png"),
                    id: row.character_id,
                    name: of(row.character_meta.name)
                });
                this.bar_tooltips.set(row.character_id, {type: 1, character_id: row.character_id});
            }
            this.bars = entries.map(row => [row.character_id, row.amount]);
        });
    }

    ngOnInit(): void {
        this.subscription_servers = this.dataService.servers.subscribe(servers => {
            this.servers = servers.map(server => {
                return {id: server.id, label: server.name + " (" + server.patch + ")"};
            });
            this.finished_loading[0] = servers.length > 0;
            if (this.finished_loading.every(item => item)) this.init_ranking();
        });
        this.subscription_hero_classes = this.dataService.hero_classes.subscribe(hero_classes => {
            this.classes = hero_classes.map(hero_class => {
                return {id: hero_class.base.id, label: hero_class.localization};
            });
            this.finished_loading[1] = hero_classes.length > 0;
            if (this.finished_loading.every(item => item)) this.init_ranking();
        });
        this.subscription_encounters = this.dataService.encounters.subscribe(encounters => {
            this.encounters = encounters.map(encounter => {
                return {id: encounter.base.id, label: encounter.localization};
            });
            this.finished_loading[2] = encounters.length > 0;
            if (this.finished_loading.every(item => item)) this.init_ranking();
        });
    }

    private init_ranking(): void {
        if (this.settingsService.check("pve_ranking")) {
            const selection_params = this.settingsService.get("pve_ranking");
            this.modes_current_selection = selection_params[0];
            this.selections_current_selection = selection_params[1];
            this.encounters_selected_items = this.encounters.filter(item => selection_params[2].includes(item.id));
            this.classes_selected_items = this.classes.filter(item => selection_params[3].includes(item.id));
            this.servers_selected_items = this.servers.filter(item => selection_params[4].includes(item.id));
        }
        this.select();
    }

    ngOnDestroy(): void {
        this.subscription_rankings?.unsubscribe();
        this.subscription_hero_classes?.unsubscribe();
        this.subscription_encounters?.unsubscribe();
        this.subscription_servers.unsubscribe();
    }

    bar_clicked(bar: [number, number]): void {

    }

    select(): void {
        if (!this.finished_loading.every(item => item))
            return;

        const selection_params = [this.modes_current_selection, this.selections_current_selection,
            this.encounters_selected_items.map(item => item.id),
            this.classes_selected_items.map(item => item.id),
            this.servers_selected_items.map(item => item.id)];
        // @ts-ignore
        this.rankingService.select(...selection_params);
        this.settingsService.set("pve_ranking", selection_params);
    }

    share(): void {
        const tiny_url = {
            type_id: 2,
            navigation_id: 8,
            payload: this.settingsService.get("pve_ranking")
        } as TinyUrl<RankingUrl>;
        this.tinyUrlService.set_tiny_url(tiny_url);
    }

}
