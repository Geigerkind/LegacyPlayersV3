import {Component} from "@angular/core";
import {ActivatedRoute} from "@angular/router";
import {CharacterViewerService} from "../../service/character_viewer";
import {CharacterViewerDto} from "../../domain_value/character_viewer_dto";

@Component({
    selector: "CharacterViewer",
    templateUrl: "./character_viewer.html",
    styleUrls: ["./character_viewer.scss"]
})
export class CharacterViewerComponent {

    character: CharacterViewerDto;

    constructor(
        private activatedRouteService: ActivatedRoute,
        private characterViewerService: CharacterViewerService
    ) {
        this.activatedRouteService.paramMap.subscribe(params => this.loadCharacter(Number(params.get('character_id'))));
    }

    private loadCharacter(character_id: number): void {
        this.characterViewerService.get_character_viewer(character_id, result => this.character = result);

        // DEBUG
        setTimeout(() => console.log(this.character), 500);
    }

    loadCharacterByHistoryId(character_history_id: number): void {
        console.log("History_changed => ", character_history_id);
        this.characterViewerService.get_character_viewer_by_history(character_history_id, this.character.character_id, result => this.character = result);
        // DEBUG
        setTimeout(() => console.log(this.character), 500);
    }
}
