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
            const history_date = params.get('character_history_date');
            if (history_date) {
                this.loadCharacterByHistoryDate(params.get('server_name'), params.get('character_name'), history_date);
                return;
            }
            this.loadCharacter(params.get('server_name'), params.get('character_name'));
        });
    }

    historyChanged(character_history_id: number): void {
        const character_history_date = this.character.history.find(option => option.value === Number(character_history_id)).label_key;
        this.routerService.navigate(['/armory/character/' + this.character.server_name + '/' + this.character.name + '/' + character_history_date]);
    }

    private loadCharacter(server_name: string, character_name: string): void {
        this.characterViewerService.get_character_viewer(server_name, character_name, result => {
            this.character = result;
            console.log(this.character);
        }, () => {
            this.routerService.navigate(['/404']);
        });
    }

    private loadCharacterByHistoryDate(server_name: string, character_name: string, character_history_date: string): void {
        this.characterViewerService.get_character_viewer_by_history_date(character_history_date, server_name, character_name, result => {
            this.character = result;
            console.log(this.character);
        }, () => {
            this.loadCharacter(server_name, character_name);
        });
    }
}
