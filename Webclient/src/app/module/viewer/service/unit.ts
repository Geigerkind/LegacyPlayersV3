import {Injectable} from "@angular/core";
import {is_creature, is_player, Unit} from "../domain_value/unit";
import {Observable, of} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {Player} from "../domain_value/player";
import {concatMap, map} from "rxjs/operators";
import {DataService} from "../../../service/data";
import {Creature} from "../domain_value/creature";
import {NPC} from "../../../domain_value/data/npc";
import {Localized} from "../../../domain_value/localized";
import {CONST_UNKNOWN_LABEL} from "../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class UnitService {

    private server_id$: number;

    constructor(
        private characterService: CharacterService,
        private dataService: DataService
    ) {
    }

    set_server_id(server_id: number): void {
        this.server_id$ = server_id;
    }

    get_unit_bg_color(unit: Unit): Observable<string> {
        // TODO: Use method to get character at the time of the raid
        if (is_player(unit)) {
            return this.characterService
                .get_basic_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => "hero_class_bg_" + character.hero_class_id.toString()));
        }
        return of("hero_class_bg_1");
    }

    get_unit_icon(unit: Unit): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_basic_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => "/assets/wow_hero_classes/c" + character.hero_class_id.toString() + "-" + character.spec_id + ".png"));
        }
        return of("/assets/wow_hero_classes/c1-0.png");
    }

    get_unit_name(unit: Unit): Observable<string> {
        if (is_player(unit))
            return this.characterService
                .get_basic_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => character.name));

        if (is_creature(unit))
            return this.get_npc_name(((unit as any).Creature as Creature).entry);

        return of(CONST_UNKNOWN_LABEL);
    }

    get_npc_name(npc_id: number): Observable<string | undefined> {
        return this.get_npc(npc_id)
            .pipe(map(npc => !npc ? CONST_UNKNOWN_LABEL : npc.localization));
    }

    get_npc(npc_id: number): Observable<Localized<NPC> | undefined> {
        if (!this.server_id$)
            return of(undefined);
        return this.dataService
            .get_server_by_id(this.server_id$)
            .pipe(concatMap(server => {
                return !server ? of(undefined) : this.dataService
                    .get_npc(server.expansion_id, npc_id);
            }));
    }

}
