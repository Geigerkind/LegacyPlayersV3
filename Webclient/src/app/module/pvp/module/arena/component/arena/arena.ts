import {Component} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";

@Component({
    selector: "Arena",
    templateUrl: "./arena.html",
    styleUrls: ["./arena.scss"]
})
export class ArenaComponent {

    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'arena',
            labelKey: "PvP.Arena.arena",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Arena.arena"}],
            col_type: 0
        },
        {index: 1, filter_name: 'team1', labelKey: "PvP.Arena.team1", type: 0, type_range: null, col_type: 0},
        {index: 2, filter_name: 'team2', labelKey: "PvP.Arena.team2", type: 0, type_range: null, col_type: 0},
        {
            index: 3,
            filter_name: 'server',
            labelKey: "PvP.Arena.server",
            type: 3,
            type_range: [{value: -1, label_key: "PvP.Arena.server"}],
            col_type: 0
        },
        {index: 4, filter_name: 'change1', labelKey: "PvP.Arena.change1", type: 1, type_range: null, col_type: 0},
        {index: 5, filter_name: 'change2', labelKey: "PvP.Arena.change2", type: 1, type_range: null, col_type: 0},
        {index: 6, filter_name: 'start', labelKey: "PvE.Search.start", type: 2, type_range: null, col_type: 2},
        {index: 7, filter_name: 'end', labelKey: "PvE.Search.end", type: 2, type_range: null, col_type: 2},
        {index: 8, filter_name: 'duration', labelKey: "PvE.Search.duration", type: 2, type_range: null, col_type: 2},
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1, 2];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor() {
    }

    onFilter(filter: any): void {

    }

}
