import {Injectable, OnDestroy} from "@angular/core";
import {is_creature, is_player, Unit} from "../domain_value/unit";
import {Observable, of, Subscription} from "rxjs";
import {CharacterService} from "../../armory/service/character";
import {Player} from "../domain_value/player";
import {concatMap, map} from "rxjs/operators";
import {DataService} from "../../../service/data";
import {Creature} from "../domain_value/creature";
import {InstanceDataService} from "./instance_data";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {NPC} from "../../../domain_value/data/npc";
import {Localized} from "../../../domain_value/localized";

@Injectable({
    providedIn: "root",
})
export class UnitService implements OnDestroy {

    private subscription: Subscription;
    private current_meta: InstanceViewerMeta;

    constructor(
        private characterService: CharacterService,
        private dataService: DataService,
        private instanceDataService: InstanceDataService
    ) {
        this.subscription = this.instanceDataService.meta.subscribe(meta => this.current_meta = meta);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get_unit_bg_color(unit: Unit): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character?.last_update ?
                    "hero_class_bg_" + character.last_update.character_info.hero_class_id.toString()
                    : "hero_class_bg_1"));
        }
        return of("hero_class_bg_1");
    }

    get_unit_icon(unit: Unit): Observable<string> {
        if (is_player(unit)) {
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character?.last_update ?
                    "/assets/wow_hero_classes/c" + character.last_update.character_info.hero_class_id.toString() + ".png"
                    : "/assets/wow_hero_classes/c1.png"));
        }
        return of("/assets/wow_hero_classes/c1.png");
    }

    get_unit_name(unit: Unit): Observable<string> {
        if (is_player(unit))
            return this.characterService
                .get_character_by_id(((unit as any).Player as Player).character_id)
                .pipe(map(character => !!character?.last_update ?
                    character.last_update.character_name
                    : "Unknown"));

        if (is_creature(unit))
            return this.get_npc_name(((unit as any).Creature as Creature).entry);

        return of("Unknown");
    }

    get_npc_name(npc_id: number): Observable<string | undefined> {
        return this.get_npc(npc_id)
            .pipe(map(npc => !npc ? "Unknown" : npc.localization));
    }

    get_npc(npc_id: number): Observable<Localized<NPC> | undefined> {
        if (!this.current_meta)
            return of(undefined);
        return this.dataService
            .get_server_by_id(this.current_meta.server_id)
            .pipe(concatMap(server => {
                return !server ? of(undefined) : this.dataService
                    .get_npc(server.expansion_id, npc_id);
            }));
    }

}
