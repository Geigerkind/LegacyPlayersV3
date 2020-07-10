import {Component} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";

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
        {index: 3, filter_name: 'end', labelKey: "PvP.Skirmish.end", type: 2, type_range: null, col_type: 2},
        {index: 4, filter_name: 'duration', labelKey: "PvP.Skirmish.duration", type: 2, type_range: null, col_type: 2},
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
