import {Component} from "@angular/core";
import {GuildViewerDto} from "../../domain_value/guild_viewer_dto";
import {ActivatedRoute, Router} from "@angular/router";
import {GuildViewerService} from "../../service/guild_viewer";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {Localized} from "../../../../../../domain_value/localized";
import {HeroClass} from "../../../../../../domain_value/hero_class";
import {DataService} from "../../../../../../service/data";
import {GuildViewerMemberDto} from "../../domain_value/guild_viewer_member_dto";
import {BodyRow} from "../../../../../../template/table/module/table_body/domain_value/body_row";

@Component({
    selector: "GuildViewer",
    templateUrl: "./guild_viewer.html",
    styleUrls: ["./guild_viewer.scss"]
})
export class GuildViewerComponent {

    guild: GuildViewerDto;

    responsiveModeWidthInPx: number = 600;
    responsiveHeadColumns: Array<number> = [0, 1];
    member_header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'hero_class',
            labelKey: "Armory.GuildViewer.hero_class",
            type: 3,
            type_range: [{value: -1, label_key: "Armory.GuildViewer.hero_class"}],
            col_type: 1
        },
        {index: 1, filter_name: 'name', labelKey: "Armory.GuildViewer.name", type: 0, type_range: null, col_type: 0},
        {index: 2, filter_name: 'rank', labelKey: "Armory.GuildViewer.rank", type: 0, type_range: null, col_type: 0},
        {index: 3, filter_name: 'last_updated', labelKey: "Armory.GuildViewer.last_update", type: 2, type_range: null, col_type: 1},
    ];
    member_body_rows: Array<BodyRow> = [];

    constructor(
        private routerService: Router,
        private activatedRouteService: ActivatedRoute,
        private dataService: DataService,
        private guildViewerService: GuildViewerService
    ) {
        this.activatedRouteService.paramMap.subscribe(params => {
            this.loadGuild(params.get('server_name'), params.get('guild_name'));
        });
    }

    private loadGuild(server_name: string, guild_name: string): void {
        this.guildViewerService.get_guild_view(server_name, guild_name, result => {
            this.setMemberBodyRows(server_name, result.member);
            this.dataService.get_all_hero_classes((hero_classes: Array<Localized<HeroClass>>) => hero_classes.forEach(hero_class => this.member_header_columns[0].type_range.push({
                value: hero_class.base.id,
                label_key: hero_class.localization
            })));
            this.guild = result;
        }, () => {
            this.routerService.navigate(['/404']);
        });
    }

    private setMemberBodyRows(server_name: string, member: Array<GuildViewerMemberDto>): void {
        this.member_body_rows = member.map(entry => {
            const body_columns: Array<BodyColumn> = [];

            body_columns.push({
                type: 3,
                content: entry.hero_class_id.toString(),
                args: null
            });
            body_columns.push({
                type: 0,
                content: entry.character_name.toString(),
                args:  {
                    server_name,
                    character_id: entry.character_id
                }
            });
            body_columns.push({
                type: 0,
                content: entry.rank.toString(),
                args: null
            });
            body_columns.push({
                type: 2,
                content: entry.last_seen.toString(),
                args: null
            });

            return {
                color: entry.faction ? '#372727' : '#272f37',
                columns: body_columns
            };
        });
    }
}
