import {Component, OnInit} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {DataService} from "../../../../../../service/data";
import {Localized} from "../../../../../../domain_value/localized";
import {InstanceMap} from "../../../../../../domain_value/instance_map";
import {AvailableServer} from "../../../../../../domain_value/available_server";
import {table_init_filter} from "../../../../../../template/table/utility/table_init_filter";
import {SkirmishSearchService} from "../../service/skirmish_search";
import {SettingsService} from "src/app/service/settings";

@Component({
    selector: "Skirmish",
    templateUrl: "./skirmish.html",
    styleUrls: ["./skirmish.scss"]
})
export class SkirmishComponent implements OnInit {

    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'map_id',
            labelKey: "PvP.Skirmish.arena",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Skirmish.arena"}],
            col_type: 0
        },
        {
            index: 1,
            filter_name: 'server_id',
            labelKey: "PvP.Skirmish.server",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Skirmish.server"}],
            col_type: 0
        },
        {index: 2, filter_name: 'start_ts', labelKey: "PvP.Skirmish.start", type: 2, type_range: null, col_type: 2},
        {index: 3, filter_name: 'end_ts', labelKey: "PvP.Skirmish.end", type: 2, type_range: null, col_type: 2}
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1, 2];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private dataService: DataService,
        private settingsService: SettingsService,
        private skirmishSearchService: SkirmishSearchService
    ) {
        this.dataService.get_all_maps_by_type(1, (instance_maps: Array<Localized<InstanceMap>>) => {
            instance_maps.forEach(map => this.header_columns[0].type_range.push({
                value: map.base.id,
                label_key: map.localization
            }));
        });
        this.dataService.get_all_servers((servers: Array<AvailableServer>) => {
            servers.forEach(server => this.header_columns[1].type_range.push({
                value: server.id,
                label_key: server.name
            }));
        });
    }

    ngOnInit(): void {
        const filter = table_init_filter(this.header_columns);
        if (!this.settingsService.check("table_filter_skirmishes_search")) {
            this.settingsService.set("table_filter_skirmishes_search", filter);
        }
    }

    onFilter(filter: any): void {
        this.skirmishSearchService.search_skirmishes(filter, (search_result) => {
            this.num_characters = search_result.num_items;
            this.body_columns = search_result.result;
        }, () => {});
    }

}
