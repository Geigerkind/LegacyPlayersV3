import {Injectable} from "@angular/core";
import {is_player, Unit} from "../domain_value/unit";
import {Observable, of} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {Player} from "../domain_value/player";
import {map} from "rxjs/operators";

@Injectable({
    providedIn: "root",
})
export class UnitService {

    constructor(
        private characterService: CharacterService
    ) {
    }

    get_unit_bg_color(unit: Unit): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character.last_update ?
                    "hero_class_bg_" + character.last_update.character_info.hero_class_id.toString()
                    : "hero_class_bg_1"));
        }
        return of("hero_class_bg_1");
    }

    get_unit_name(unit: Unit): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character.last_update ?
                    character.last_update.character_name
                    : "Unknown"));
        }
        return of("TODO: Creature Name");
    }

}
