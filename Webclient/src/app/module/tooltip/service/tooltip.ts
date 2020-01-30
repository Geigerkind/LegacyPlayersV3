import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";

@Injectable({
    providedIn: "root",
})
export class TooltipService {
    private static readonly URL_TOOLTIP: string = "/tooltip";
    private static readonly URL_ITEM_TOOLTIP: string = "/item";
    private static readonly URL_SPELL_TOOLTIP: string = "/spell";
    private static readonly URL_CHARACTER_TOOLTIP: string = "/character";

    constructor(
        private apiService: APIService
    ) {
    }

    loadCharacterTooltip(character_id: number, on_success: any): void {
        this.apiService.get(TooltipService.URL_TOOLTIP + TooltipService.URL_CHARACTER_TOOLTIP + "/" + character_id, on_success);
    }
}
