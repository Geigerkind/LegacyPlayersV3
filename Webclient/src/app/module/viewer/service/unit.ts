import {Injectable} from "@angular/core";
import {get_creature_entry, get_unit_id, get_unit_owner, is_creature, is_player, Unit} from "../domain_value/unit";
import {combineLatest, Observable, of} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {map} from "rxjs/operators";
import {DataService} from "../../../service/data";
import {NPC} from "../../../domain_value/data/npc";
import {Localized} from "../../../domain_value/localized";
import {CONST_UNKNOWN_LABEL} from "../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class UnitService {

    private server_id$: number;
    private expansion_id$: number;

    constructor(
        private characterService: CharacterService,
        private dataService: DataService
    ) {
    }

    set_server_id(server_id: number): void {
        this.server_id$ = server_id;
    }

    set_expansion_id(expansion_id: number): void {
        this.expansion_id$ = expansion_id;
    }

    get_unit_bg_color(unit: Unit, timestamp: number): Observable<string> {
        if (!!unit) {
            if (is_player(unit, false)) {
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false), timestamp)
                    .pipe(map(character => "hero_class_bg_" + character.hero_class_id.toString()));
            }
        }
        return of("hero_class_bg_1");
    }

    get_unit_icon(unit: Unit, timestamp: number): Observable<string> {
        if (!!unit) {
            if (is_player(unit, false)) {
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false), timestamp)
                    .pipe(map(character => "/assets/wow_hero_classes/c" + character.hero_class_id.toString() + "-" + character.spec_id + ".png"));
            }
        }
        return of("/assets/wow_hero_classes/c1-0.png");
    }

    get_unit_name(unit: Unit, timestamp: number): Observable<string> {
        if (!!unit) {
            if (is_player(unit, false))
                return this.characterService
                    .get_basic_character_by_id(get_unit_id(unit, false), timestamp)
                    .pipe(map(character => character.name));

            if (is_creature(unit, false)) {
                const creatureEntry = get_creature_entry(unit);
                const npc = this.get_npc_name(creatureEntry);
                if (!!unit[3]) {
                    const owner = this.get_unit_name(get_unit_owner(unit), timestamp);
                    return combineLatest([npc, owner]).pipe(map(([npc_name, owner_name]) =>
                        npc_name + " (" + owner_name + ")"));
                }
                return npc;
            }
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
        return this.dataService.get_npc(this.expansion_id$, npc_id);
    }

}
