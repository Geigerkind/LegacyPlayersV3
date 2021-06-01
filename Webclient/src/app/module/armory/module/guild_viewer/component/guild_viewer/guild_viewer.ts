import {Component} from "@angular/core";
import {GuildViewerDto} from "../../domain_value/guild_viewer_dto";
import {ActivatedRoute, Router} from "@angular/router";
import {GuildViewerService} from "../../service/guild_viewer";
import {SpeedKillService} from "../../../../../pve/module/speed_kill/service/speed_kill";
import {SpeedRunService} from "../../../../../pve/module/speed_run/service/speed_run";

@Component({
    selector: "GuildViewer",
    templateUrl: "./guild_viewer.html",
    styleUrls: ["./guild_viewer.scss"],
    providers: [
        SpeedKillService,
        SpeedRunService
    ]
})
export class GuildViewerComponent {

    guild: GuildViewerDto;
    server_name: string = '';

    constructor(
        private routerService: Router,
        private activatedRouteService: ActivatedRoute,
        private guildViewerService: GuildViewerService,
    ) {
        this.activatedRouteService.paramMap.subscribe(params => {
            this.server_name = params.get('server_name');
            this.guildViewerService.get_guild_view(this.server_name, params.get('guild_name'), result => {
                this.guild = result;
            }, () => {
                this.routerService.navigate(['/404']);
            });
        });
    }
}
