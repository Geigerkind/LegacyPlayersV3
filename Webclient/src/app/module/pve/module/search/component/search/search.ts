import {Component, OnInit} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {DataService} from "../../../../../../service/data";
import {AvailableServer} from "../../../../../../domain_value/available_server";
import {Localized} from "../../../../../../domain_value/localized";
import {InstanceMap} from "../../../../../../domain_value/instance_map";
import {Difficulty} from "../../../../../../domain_value/difficulty";
import {RaidSearchService} from "../../service/raid_search";
import {table_init_filter} from "../../../../../../template/table/utility/table_init_filter";
import {SettingsService} from "../../../../../../service/settings";

@Component({
    selector: "Search",
    templateUrl: "./search.html",
    styleUrls: ["./search.scss"],
    providers: [RaidSearchService]
})
export class SearchComponent implements OnInit {
    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'map_id',
            labelKey: "PvE.Search.raid",
            type: 3,
            type_range: [{value: -1, label_key: "PvE.Search.raid"}],
            col_type: 0
        },
        {
            index: 1,
            filter_name: 'map_difficulty',
            labelKey: "PvE.Search.difficulty",
            type: 3,
            type_range: [{value: -1, label_key: "PvE.Search.difficulty"}],
            col_type: 2
        },
        {index: 2, filter_name: 'guild', labelKey: "PvE.Search.guild", type: 0, type_range: null, col_type: 0},
        {
            index: 3,
            filter_name: 'server_id',
            labelKey: "PvE.Search.server",
            type: 3,
            type_range: [{value: -1, label_key: "PvE.Search.server"}],
            col_type: 0
        },
        {index: 4, filter_name: 'start_ts', labelKey: "PvE.Search.start", type: 2, type_range: null, col_type: 2},
        {index: 5, filter_name: 'end_ts', labelKey: "PvE.Search.end", type: 2, type_range: null, col_type: 2}
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1, 2];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private dataService: DataService,
        private searchService: RaidSearchService,
        private settingsService: SettingsService
    ) {
        this.dataService.get_all_maps_by_type(0, (instance_maps: Array<Localized<InstanceMap>>) => {
            instance_maps.forEach(map => this.header_columns[0].type_range.push({
                value: map.base.id,
                label_key: map.localization
            }));
        });
        this.dataService.get_all_difficulties((difficulties: Array<Localized<Difficulty>>) => {
            difficulties.forEach(difficulty => this.header_columns[1].type_range.push({
                value: difficulty.base.id,
                label_key: difficulty.localization
            }));
        });
        this.dataService.get_all_servers((servers: Array<AvailableServer>) => {
            servers.forEach(server => this.header_columns[3].type_range.push({
                value: server.id,
                label_key: server.name
            }));
        });
    }

    ngOnInit(): void {
        const filter = table_init_filter(this.header_columns);
        if (!this.settingsService.check("table_filter_raids_search")) {
            filter.last_updated.sorting = false;
            this.settingsService.set("table_filter_raids_search", filter);
        }
    }

    onFilter(filter: any): void {
        this.searchService.search_raids(filter, (search_result) => {
            this.num_characters = search_result.num_items;
            this.body_columns = search_result.result;
        }, () => {});
    }

}
