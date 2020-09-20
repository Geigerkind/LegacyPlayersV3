import {Injectable} from "@angular/core";
import {get_creature_entry, get_unit_id, is_creature, is_player, Unit} from "../domain_value/unit";
import {Observable, of} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {concatMap, map} from "rxjs/operators";
import {DataService} from "../../../service/data";
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
        if (!!unit) {
            // TODO: Use method to get character at the time of the raid
            if (is_player(unit, false)) {
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false))
                    .pipe(map(character => "hero_class_bg_" + character.hero_class_id.toString()));
            }
        }
        return of("hero_class_bg_1");
    }

    get_unit_icon(unit: Unit): Observable<string> {
        if (!!unit) {
            if (is_player(unit, false)) {
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false))
                    .pipe(map(character => "/assets/wow_hero_classes/c" + character.hero_class_id.toString() + "-" + character.spec_id + ".png"));
            }
        }
        return of("/assets/wow_hero_classes/c1-0.png");
    }

    get_unit_name(unit: Unit): Observable<string> {
        if (!!unit) {
            if (is_player(unit, false))
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false))
                    .pipe(map(character => character.name));

            if (is_creature(unit, false))
                return this.get_npc_name(get_creature_entry(unit));
        }
        return of(CONST_UNKNOWN_LABEL);
    }

    is_unit_boss(unit: Unit): Observable<boolean> {
        if (!!unit && is_creature(unit, false)) {
            return this.get_npc(get_creature_entry(unit))
                .pipe(map(npc => !!npc?.base.is_boss));
        }
        return of(false);
    }

    get_npc_name(npc_id: number): Observable<string | undefined> {
        return this.get_npc(npc_id)
            .pipe(map(npc => !npc ? CONST_UNKNOWN_LABEL : npc.localization));
    }

    get_npc(npc_id: number): Observable<Localized<NPC> | undefined> {
        if (npc_id === 65535) {
            return of({
                localization: "Pet",
                base: {
                    expansion_id: 0,
                    id: npc_id,
                    localization_id: 0,
                    is_boss: false,
                    friend: 0,
                    family: 0
                }
            });
        }

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
