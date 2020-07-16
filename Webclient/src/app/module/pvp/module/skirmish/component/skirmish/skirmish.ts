import {Component} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {DataService} from "../../../../../../service/data";
import {Localized} from "../../../../../../domain_value/localized";
import {InstanceMap} from "../../../../../../domain_value/instance_map";
import {AvailableServer} from "../../../../../../domain_value/available_server";

@Component({
    selector: "Skirmish",
    templateUrl: "./skirmish.html",
    styleUrls: ["./skirmish.scss"]
})
export class SkirmishComponent {

    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'arena',
            labelKey: "PvP.Skirmish.arena",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Skirmish.arena"}],
            col_type: 0
        },
        {
            index: 1,
            filter_name: 'server',
            labelKey: "PvP.Skirmish.server",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Skirmish.server"}],
            col_type: 0
        },
        {index: 2, filter_name: 'start', labelKey: "PvP.Skirmish.start", type: 2, type_range: null, col_type: 2},
        {index: 3, filter_name: 'end', labelKey: "PvP.Skirmish.end", type: 2, type_range: null, col_type: 2}
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1, 2];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private dataService: DataService
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

    onFilter(filter: any): void {

    }

}
