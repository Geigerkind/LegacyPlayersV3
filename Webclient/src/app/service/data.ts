import {Injectable} from "@angular/core";
import {APIService} from "./api";
import {Observable, of, Subject} from "rxjs";
import {Localized} from "../domain_value/localized";
import {InstanceMap} from "../domain_value/instance_map";
import {map, shareReplay} from "rxjs/operators";
import {AvailableServer} from "../domain_value/available_server";
import {NPC} from "../domain_value/data/npc";

@Injectable({
    providedIn: "root",
})
export class DataService {
    private static readonly CACHE_SIZE: number = 1000;

    private static readonly URL_DATA_SERVER: string = '/data/server';
    private static readonly URL_DATA_RACE_LOCALIZED: string = '/data/race/localized';
    private static readonly URL_DATA_HERO_CLASS_LOCALIZED: string = '/data/hero_class/localized';
    private static readonly URL_DATA_DIFFICULTY_LOCALIZED: string = '/data/difficulty/localized';
    private static readonly URL_DATA_MAP_LOCALIZED: string = '/data/map/localized';

    private static readonly URL_DATA_NPC_LOCALIZED: string = '/data/npc/localized/:expansion_id/:npc_id';

    private maps$: Subject<Array<Localized<InstanceMap>>>;
    private servers$: Subject<Array<AvailableServer>>;

    private npcs$: Map<number, Map<number, Subject<Localized<NPC>>>> = new Map();

    constructor(
        private apiService: APIService
    ) {
    }

    get servers(): Observable<Array<AvailableServer>> {
        if (!this.servers$) {
            this.servers$ = new Subject();
            this.servers$.pipe(shareReplay(DataService.CACHE_SIZE));
            this.apiService.get(DataService.URL_DATA_SERVER, (servers => this.servers$.next(servers)));
        }
        return this.servers$.asObservable();
    }

    get_all_races(on_success: any): void {
        this.apiService.get(DataService.URL_DATA_RACE_LOCALIZED, on_success);
    }

    get_all_hero_classes(on_success: any): void {
        this.apiService.get(DataService.URL_DATA_HERO_CLASS_LOCALIZED, on_success);
    }

    get_all_difficulties(on_success: any): void {
        this.apiService.get(DataService.URL_DATA_DIFFICULTY_LOCALIZED, on_success);
    }

    get maps(): Observable<Array<Localized<InstanceMap>>> {
        if (!this.maps$) {
            this.maps$ = new Subject();
            this.maps$.pipe(shareReplay(DataService.CACHE_SIZE));
            this.apiService.get(DataService.URL_DATA_MAP_LOCALIZED, (maps => this.maps$.next(maps)));
        }
        return this.maps$.asObservable();
    }

    get_npc(expansion_id: number, npc_id: number): Observable<Localized<NPC>> {
        if (!this.npcs$.has(expansion_id))
            this.npcs$.set(expansion_id, new Map());
        const expansion = this.npcs$.get(expansion_id);
        if (expansion.has(npc_id))
            return expansion.get(npc_id).asObservable();

        const subject = new Subject<Localized<NPC>>();
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
