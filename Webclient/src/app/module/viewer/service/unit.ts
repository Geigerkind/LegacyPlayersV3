import {Injectable} from "@angular/core";
import {is_creature, is_player, Unit} from "../domain_value/unit";
import {Observable, of} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {Player} from "../domain_value/player";
import {concatMap, map} from "rxjs/operators";
import {DataService} from "../../../service/data";
import {Creature} from "../domain_value/creature";

@Injectable({
    providedIn: "root",
})
export class UnitService {

    constructor(
        private characterService: CharacterService,
        private dataService: DataService
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

    get_unit_name(unit: Unit, server_id: number): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character.last_update ?
                    character.last_update.character_name
                    : "Unknown"));
        }

        if (is_creature(unit)) {
            return this.dataService
                .get_server_by_id(server_id)
                .pipe(concatMap(server => this.dataService
                    .get_npc(server.expansion_id, ((unit as any).Creature as Creature).entry)
                    .pipe(map(npc => npc.localization))));
        }

        return of("Unknown");
    }

}
