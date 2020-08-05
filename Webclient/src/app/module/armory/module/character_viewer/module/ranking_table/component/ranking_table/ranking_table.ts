import {Component, Input, OnDestroy, OnInit} from "@angular/core";
import {HeaderColumn} from "../../../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../../../template/table/module/table_body/domain_value/body_column";
import {RankingTableService} from "../../service/ranking_table";
import {Subscription} from "rxjs";

@Component({
    selector: "RankingTable",
    templateUrl: "./ranking_table.html",
    styleUrls: ["./ranking_table.scss"],
    providers: [
        RankingTableService
    ]
})
export class RankingTableComponent implements OnDestroy, OnInit {

    private subscription: Subscription;

    header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'npc_id',
            labelKey: "Armory.Viewer.RankingTable.boss_name",
            type: 0,
            type_range: null,
            col_type: 0
        },
        {
            index: 1,
            filter_name: 'dps',
            labelKey: "Armory.Viewer.RankingTable.dps",
            type: 1,
            type_range: null,
            col_type: 2
        },
        {
            index: 2,
            filter_name: 'hps',
            labelKey: "Armory.Viewer.RankingTable.hps",
            type: 1,
            type_range: null,
            col_type: 2
        },
        {
            index: 3,
            filter_name: 'tps',
            labelKey: "Armory.Viewer.RankingTable.tps",
            type: 1,
            type_range: null,
            col_type: 2
        }
    ];
    body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = true;
    responsiveHeadColumns: Array<number> = [0, 1, 2];
    responsiveModeWidthInPx: number = 600;
    num_characters: number = 0;

    @Input() character_id: number;

    constructor(
        private rankingTableService: RankingTableService
    ) {
    }

    ngOnInit(): void {
        this.rankingTableService.get_rows(this.character_id).subscribe(rows => this.body_columns = rows);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

}
