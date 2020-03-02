import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class CharacterViewerService {
    private static readonly URL_ARMORY_CHARACTER_VIEWER: string = "/armory/character_viewer";
    private static readonly URL_ARMORY_CHARACTER_VIEWER_BY_HISTORY: string = "/armory/character_viewer";

    constructor(
        private apiService: APIService
    ) {
    }

    get_character_viewer(server_name: string, character_name: string, on_success: any, on_failure: any): void {
        this.apiService.get(CharacterViewerService.URL_ARMORY_CHARACTER_VIEWER + "/" + server_name + "/" + character_name,
                result => on_success.call(on_success, result), on_failure);
    }

    get_character_viewer_by_history(character_history_id: number, server_name: string, character_name: string, on_success: any, on_failure: any): void {
        this.apiService.get(CharacterViewerService.URL_ARMORY_CHARACTER_VIEWER_BY_HISTORY + "/" + server_name + "/" + character_name + "/" + character_history_id,
            result => on_success.call(on_success, result), on_failure);
    }
}
