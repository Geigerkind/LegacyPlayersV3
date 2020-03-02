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
                this.loadCharacterByHistoryId(params.get('server_name'), params.get('character_name'), Number(history_id));
                return;
            }
            this.loadCharacter(params.get('server_name'), params.get('character_name'));
        });
    }

    historyChanged(character_history_id: number): void {
        this.routerService.navigate(['/armory/character/' + this.character.server_name + '/' + this.character.name + '/' + character_history_id]);
    }

    private loadCharacter(server_name: string, character_name: string): void {
        this.characterViewerService.get_character_viewer(server_name, character_name, result => this.character = result, () => {
            this.routerService.navigate(['/404']);
        });
    }

    private loadCharacterByHistoryId(server_name: string, character_name: string, character_history_id: number): void {
        this.characterViewerService.get_character_viewer_by_history(character_history_id, server_name, character_name, result => this.character = result, () => {
            this.loadCharacter(server_name, character_name);
        });
    }
}
