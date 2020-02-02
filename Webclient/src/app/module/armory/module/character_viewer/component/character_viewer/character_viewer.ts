import {Component} from "@angular/core";
import {ActivatedRoute, Router} from "@angular/router";
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
        private routerService: Router,
        private activatedRouteService: ActivatedRoute,
        private characterViewerService: CharacterViewerService
    ) {
        this.activatedRouteService.paramMap.subscribe(params => {
            const history_id = params.get('character_history_id');
            if (history_id) {
                this.loadCharacterByHistoryId(Number(params.get('character_id')), Number(history_id));
                return;
            }
            this.loadCharacter(Number(params.get('character_id')));
        });
    }

    historyChanged(character_history_id: number): void {
        this.routerService.navigate(['/armory/character/' + this.character.character_id + '/' + character_history_id]);
    }

    private loadCharacter(character_id: number): void {
        this.characterViewerService.get_character_viewer(character_id, result => this.character = result);
    }

    private loadCharacterByHistoryId(character_id: number, character_history_id: number): void {
        this.characterViewerService.get_character_viewer_by_history(character_history_id, character_id, result => this.character = result);
    }
}
