import {Injectable} from "@angular/core";
import {APIService} from "./api";
import {BehaviorSubject, Observable} from "rxjs";
import {Localized} from "../domain_value/localized";
import {InstanceMap} from "../domain_value/instance_map";
import {map} from "rxjs/operators";
import {AvailableServer} from "../domain_value/available_server";
import {NPC} from "../domain_value/data/npc";
import {SettingsService} from "./settings";
import {Race} from "../domain_value/race";
import {HeroClass} from "../domain_value/hero_class";
import {Difficulty} from "../domain_value/difficulty";

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

    private maps$: BehaviorSubject<Array<Localized<InstanceMap>>>;
    private servers$: BehaviorSubject<Array<AvailableServer>>;
    private races$: BehaviorSubject<Array<Localized<Race>>>;
    private hero_classes$: BehaviorSubject<Array<Localized<HeroClass>>>;
    private difficulties$: BehaviorSubject<Array<Localized<Difficulty>>>;

    private npcs$: Map<number, Map<number, BehaviorSubject<Localized<NPC>>>> = new Map();

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
    }

    get servers(): Observable<Array<AvailableServer>> {
        this.servers$ = this.settingsService.init_or_load_behavior_subject("data_service_servers", 1, this.servers$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_SERVER, callback));
        return this.servers$.asObservable();
    }

    get races(): Observable<Array<Localized<Race>>> {
        this.races$ = this.settingsService.init_or_load_behavior_subject("data_service_races", 7, this.races$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_RACE_LOCALIZED, callback));
        return this.races$.asObservable();
    }

    get hero_classes(): Observable<Array<Localized<HeroClass>>> {
        this.hero_classes$ = this.settingsService.init_or_load_behavior_subject("data_service_hero_classes", 7, this.hero_classes$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_HERO_CLASS_LOCALIZED, callback));
        return this.hero_classes$.asObservable();
    }

    get difficulties(): Observable<Array<Localized<Difficulty>>> {
        this.difficulties$ = this.settingsService.init_or_load_behavior_subject("data_service_difficulties", 7, this.difficulties$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_DIFFICULTY_LOCALIZED, callback));
        return this.difficulties$.asObservable();
    }

    get maps(): Observable<Array<Localized<InstanceMap>>> {
        this.maps$ = this.settingsService.init_or_load_behavior_subject("data_service_maps", 7, this.maps$, [],
            (callback) => this.apiService.get(DataService.URL_DATA_MAP_LOCALIZED, callback));
        return this.maps$.asObservable();
    }

    get_npc(expansion_id: number, npc_id: number): Observable<Localized<NPC>> {
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
            npc => subject.next(npc));

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
