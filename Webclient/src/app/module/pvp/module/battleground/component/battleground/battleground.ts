import {Component, OnInit} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {DataService} from "../../../../../../service/data";
import {Localized} from "../../../../../../domain_value/localized";
import {InstanceMap} from "../../../../../../domain_value/instance_map";
import {AvailableServer} from "../../../../../../domain_value/available_server";
import {SettingsService} from "src/app/service/settings";
import {BattlegroundSearchService} from "../../service/battleground_search";
import {table_init_filter} from "../../../../../../template/table/utility/table_init_filter";

@Component({
    selector: "Battleground",
    templateUrl: "./battleground.html",
    styleUrls: ["./battleground.scss"]
})
export class BattlegroundComponent implements OnInit {

    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'map_id',
            labelKey: "PvP.Battleground.battleground",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Battleground.battleground"}],
            col_type: 0
        },
        {
            index: 1,
            filter_name: 'server_id',
            labelKey: "PvP.Battleground.server",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Battleground.server"}],
            col_type: 0
        },
        {index: 2, filter_name: 'score_alliance', labelKey: "PvP.Battleground.score_alliance", type: 1, type_range: null, col_type: 0},
        {index: 3, filter_name: 'score_horde', labelKey: "PvP.Battleground.score_horde", type: 1, type_range: null, col_type: 0},
        {index: 4, filter_name: 'start_ts', labelKey: "PvP.Battleground.start", type: 2, type_range: null, col_type: 2},
        {index: 5, filter_name: 'end_ts', labelKey: "PvP.Battleground.end", type: 2, type_range: null, col_type: 2}
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 4];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private dataService: DataService,
        private settingsService: SettingsService,
        private battlegroundSearchService: BattlegroundSearchService
    ) {
        this.dataService.get_all_maps_by_type(2, (instance_maps: Array<Localized<InstanceMap>>) => {
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
        if (!this.settingsService.check("table_filter_battlegrounds_search")) {
            filter.last_updated.sorting = false;
            this.settingsService.set("table_filter_battlegrounds_search", filter);
        }
    }

    onFilter(filter: any): void {
        this.battlegroundSearchService.search_battlegrounds(filter, (search_result) => {
            this.num_characters = search_result.num_items;
            this.body_columns = search_result.result;
        }, () => {});
    }

}
