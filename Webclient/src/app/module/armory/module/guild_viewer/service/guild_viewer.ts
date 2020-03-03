import {Injectable} from "@angular/core";
import { APIService } from 'src/app/service/api';

@Injectable({
    providedIn: "root",
})
export class GuildViewerService {
    private static readonly URL_ARMORY_GUILD_VIEWER: string = "/armory/guild_viewer";

    constructor(
        private apiService: APIService
    ) {
    }

    get_guild_view(server_name: string, guild_name: string, on_success: any, on_failure: any): void {
        this.apiService.get(GuildViewerService.URL_ARMORY_GUILD_VIEWER + "/" + server_name + "/" + guild_name,
            result => on_success.call(on_success, result), on_failure);
    }
}

