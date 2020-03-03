import {Component} from "@angular/core";
import {GuildViewerDto} from "../../domain_value/guild_viewer_dto";
import {ActivatedRoute, Router} from "@angular/router";
import {GuildViewerService} from "../../service/guild_viewer";

@Component({
    selector: "GuildViewer",
    templateUrl: "./guild_viewer.html",
    styleUrls: ["./guild_viewer.scss"]
})
export class GuildViewerComponent {

    guild: GuildViewerDto;

    constructor(
        private routerService: Router,
        private activatedRouteService: ActivatedRoute,
        private guildViewerService: GuildViewerService
    ) {
        this.activatedRouteService.paramMap.subscribe(params => {
            this.loadGuild(params.get('server_name'), params.get('guild_name'));
        });
    }

    private loadGuild(server_name: string, guild_name: string): void {
        this.guildViewerService.get_guild_view(server_name, guild_name, result => this.guild = result, () => {
            this.routerService.navigate(['/404']);
        });
    }

}
