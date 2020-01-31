import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class CharacterViewerService {
    private static readonly URL_ARMORY_CHARACTER_VIEWER: string = "/armory/character_viewer";

    constructor(
        private apiService: APIService
    ) {
    }

    get_character_viewer(character_id: number, on_success: any): void {
        this.apiService.get(CharacterViewerService.URL_ARMORY_CHARACTER_VIEWER + "/" + character_id,
                result => on_success.call(on_success, result));
    }
}
