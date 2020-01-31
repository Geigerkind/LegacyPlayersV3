import {Component} from "@angular/core";
import {ActivatedRoute} from "@angular/router";
import {CharacterViewerService} from "../../service/character_viewer";

@Component({
    selector: "CharacterViewer",
    templateUrl: "./character_viewer.html",
    styleUrls: ["./character_viewer.scss"]
})
export class CharacterViewerComponent {

    character: any; // TODO TYPE

    constructor(
        private activatedRouteService: ActivatedRoute,
        private characterViewerService: CharacterViewerService
    ) {
        this.activatedRouteService.paramMap.subscribe(params => this.loadCharacter(Number(params.get('character_id'))));
    }

    private loadCharacter(character_id: number): void {
        this.characterViewerService.get_character(character_id, result => this.character = result);

        // DEBUG
        setTimeout(() => console.log(this.character), 500);
    }
}
