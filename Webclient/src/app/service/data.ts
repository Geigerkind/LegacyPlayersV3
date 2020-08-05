import {Injectable} from "@angular/core";
import {APIService} from "./api";
import {BehaviorSubject, Observable, of} from "rxjs";
import {Localized} from "../domain_value/localized";
import {InstanceMap} from "../domain_value/instance_map";
import {map} from "rxjs/operators";
import {AvailableServer} from "../domain_value/available_server";
import {NPC} from "../domain_value/data/npc";
import {SettingsService} from "./settings";
import {Race} from "../domain_value/race";
import {HeroClass} from "../domain_value/hero_class";
import {Difficulty} from "../domain_value/difficulty";
import {
    get_behavior_subject_map_from_nested_array,
    create_array_from_nested_behavior_subject_map
} from "../stdlib/map_persistance";
import {BasicItem} from "../domain_value/data/basic_item";
import {BasicSpell} from "../domain_value/data/basic_spell";

@Injectable({
    providedIn: "root",
})
export class DataService {
    private static readonly URL_DATA_SERVER: string = '/data/server';
    private static readonly URL_DATA_RACE_LOCALIZED: string = '/data/race/localized';
    private static readonly URL_DATA_HERO_CLASS_LOCALIZED: string = '/data/hero_class/localized';
    private static readonly URL_DATA_DIFFICULTY_LOCALIZED: string = '/data/difficulty/localized';
    private static readonly URL_DATA_MAP_LOCALIZED: string = '/data/map/localized';
    private static readonly URL_DATA_NPC_LOCALIZED: string = '/data/npc/localized/:expansion_id/:npc_id';
    private static readonly URL_DATA_BOSS_NPCS: string = '/data/npc/localized/bosses';
    private static readonly URL_DATA_BASIC_ITEM_LOCALIZED: string = '/data/item/localized/basic_item/:expansion_id/:item_id';
    private static readonly URL_DATA_BASIC_SPELL_LOCALIZED: string = '/data/spell/localized/basic_spell/:expansion_id/:spell_id';

    private maps$: BehaviorSubject<Array<Localized<InstanceMap>>>;
    private bosses$: BehaviorSubject<Array<Localized<NPC>>>;
    private servers$: BehaviorSubject<Array<AvailableServer>>;
    private races$: BehaviorSubject<Array<Localized<Race>>>;
    private hero_classes$: BehaviorSubject<Array<Localized<HeroClass>>>;
    private difficulties$: BehaviorSubject<Array<Localized<Difficulty>>>;

    private npcs$: Map<number, Map<number, BehaviorSubject<Localized<NPC>>>>;
    private basic_items$: Map<number, Map<number, BehaviorSubject<Localized<BasicItem>>>>;
    private basic_spells$: Map<number, Map<number, BehaviorSubject<Localized<BasicSpell>>>>;

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
    }

    get servers(): Observable<Array<AvailableServer>> {
        this.servers$ = this.settingsService.init_or_load_behavior_subject("data_service_servers", 1, this.servers$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_SERVER, callback));
        return this.servers$.asObservable().pipe(map(result => result.sort((left, right) => left.id - right.id)));
    }

    get races(): Observable<Array<Localized<Race>>> {
        this.races$ = this.settingsService.init_or_load_behavior_subject("data_service_races", 7, this.races$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_RACE_LOCALIZED, callback));
        return this.races$.asObservable().pipe(map(result => result.sort((left, right) => left.base.id - right.base.id)));
    }

    get hero_classes(): Observable<Array<Localized<HeroClass>>> {
        this.hero_classes$ = this.settingsService.init_or_load_behavior_subject("data_service_hero_classes", 7, this.hero_classes$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_HERO_CLASS_LOCALIZED, callback));
        return this.hero_classes$.asObservable().pipe(map(result => result.sort((left, right) => left.base.id - right.base.id)));
    }

    get difficulties(): Observable<Array<Localized<Difficulty>>> {
        this.difficulties$ = this.settingsService.init_or_load_behavior_subject("data_service_difficulties", 7, this.difficulties$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_DIFFICULTY_LOCALIZED, callback));
        return this.difficulties$.asObservable().pipe(map(result => result.sort((left, right) => left.base.id - right.base.id)));
    }

    get maps(): Observable<Array<Localized<InstanceMap>>> {
        this.maps$ = this.settingsService.init_or_load_behavior_subject("data_service_maps", 7, this.maps$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_MAP_LOCALIZED, callback));
        return this.maps$.asObservable().pipe(map(result => result.sort((left, right) => left.base.id - right.base.id)));
    }

    get boss_npcs(): Observable<Array<Localized<NPC>>> {
        this.bosses$ = this.settingsService.init_or_load_behavior_subject("data_service_bosses", 7, this.bosses$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_BOSS_NPCS, callback));
        return this.bosses$.asObservable().pipe(map(result => result.sort((left, right) => left.base.id - right.base.id)));
    }

    get_npc(expansion_id: number, npc_id: number): Observable<Localized<NPC>> {
        if (!this.npcs$)
            this.npcs$ = get_behavior_subject_map_from_nested_array(this.settingsService.get_or_set_with_expiration("data_service_npcs", [], 7));

        if (!this.npcs$.has(expansion_id))
            this.npcs$.set(expansion_id, new Map());
        const expansion = this.npcs$.get(expansion_id);
        if (expansion.has(npc_id))
            return expansion.get(npc_id).asObservable();

        const subject = new BehaviorSubject<Localized<NPC>>(undefined);
        expansion.set(npc_id, subject);

        this.apiService.get(DataService.URL_DATA_NPC_LOCALIZED
                .replace(":expansion_id", expansion_id.toString())
                .replace(":npc_id", npc_id.toString()),
            npc => {
                subject.next(npc);
                this.settingsService.set_with_expiration("data_service_npcs", create_array_from_nested_behavior_subject_map(this.npcs$), 7);
            });

        return subject;
    }

    get_localized_basic_item(expansion_id: number, item_id: number): Observable<Localized<BasicItem>> {
        if (!this.basic_items$)
            this.basic_items$ = get_behavior_subject_map_from_nested_array(this.settingsService.get_or_set_with_expiration("data_service_basic_items", [], 7));

        if (!this.basic_items$.has(expansion_id))
            this.basic_items$.set(expansion_id, new Map());
        const expansion = this.basic_items$.get(expansion_id);
        if (expansion.has(item_id))
            return expansion.get(item_id).asObservable();

        const subject = new BehaviorSubject<Localized<BasicItem>>(undefined);
        expansion.set(item_id, subject);

        this.apiService.get(DataService.URL_DATA_BASIC_ITEM_LOCALIZED
                .replace(":expansion_id", expansion_id.toString())
                .replace(":item_id", item_id.toString()),
            item => {
                subject.next(item);
                this.settingsService.set_with_expiration("data_service_basic_items", create_array_from_nested_behavior_subject_map(this.basic_items$), 7);
            });

        return subject;
    }

    get_localized_basic_spell(expansion_id: number, spell_id: number): Observable<Localized<BasicSpell>> {
        if (!this.basic_spells$)
            this.basic_spells$ = get_behavior_subject_map_from_nested_array(this.settingsService.get_or_set_with_expiration("data_service_basic_spells", [], 7));

        if (!this.basic_spells$.has(expansion_id))
            this.basic_spells$.set(expansion_id, new Map());
        const expansion = this.basic_spells$.get(expansion_id);
        if (expansion.has(spell_id))
            return expansion.get(spell_id).asObservable();

        const subject = new BehaviorSubject<Localized<BasicSpell>>(undefined);
        expansion.set(spell_id, subject);

        this.apiService.get(DataService.URL_DATA_BASIC_SPELL_LOCALIZED
                .replace(":expansion_id", expansion_id.toString())
                .replace(":spell_id", spell_id.toString()),
            item => {
                subject.next(item);
                this.settingsService.set_with_expiration("data_service_basic_spells", create_array_from_nested_behavior_subject_map(this.basic_spells$), 7);
            });

        return subject;
    }

    get_maps_by_type(map_type: number): Observable<Array<Localized<InstanceMap>>> {
        return this.maps
            .pipe(map(maps => maps.filter(inner_map => inner_map.base.map_type === map_type)));
    }

    get_map_name_by_id(map_id: number): Observable<string> {
        return this.maps
            .pipe(
                map(maps => maps.find(inner_map => inner_map.base.id === map_id)),
                map(inner_map => !!inner_map ? inner_map.localization : '')
            );
    }

    get_server_by_id(server_id: number): Observable<AvailableServer> {
        return this.servers
            .pipe(map(servers => servers.find(server => server.id === server_id)));
    }
}
