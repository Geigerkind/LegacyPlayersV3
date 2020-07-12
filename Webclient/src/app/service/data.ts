import {Injectable} from "@angular/core";
import {APIService} from "./api";

@Injectable({
    providedIn: "root",
})
export class DataService {
    private static readonly URL_DATA_SERVER: string = '/data/server';
    private static readonly URL_DATA_RACE_LOCALIZED: string = '/data/race/localized';
    private static readonly URL_DATA_HERO_CLASS_LOCALIZED: string = '/data/hero_class/localized';
    private static readonly URL_DATA_DIFFICULTY_LOCALIZED: string = '/data/difficulty/localized';
    private static readonly URL_DATA_MAP_BY_TYPE_LOCALIZED: string = '/data/map/localized/by_type';

    constructor(
        private apiService: APIService
    ) {
    }

    get_all_servers(on_success: any): void {
        this.apiService.get(DataService.URL_DATA_SERVER, on_success);
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

    get_all_maps_by_type(map_type: number, on_success: any): void {
        this.apiService.get(DataService.URL_DATA_MAP_BY_TYPE_LOCALIZED + "/" + map_type, on_success);
    }
}
