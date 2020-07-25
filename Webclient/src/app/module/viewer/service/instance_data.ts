import {Injectable} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";

@Injectable({
    providedIn: "root",
})
export class InstanceDataService {
    private static INSTANCE_EXPORT_URL: string = "/instance/export/:instance_meta_id/:event_type";
    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";

    private instance_meta_id$: number;

    private spell_casts$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private deaths$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private combat_states$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private loot$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private positions$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private powers$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private aura_applications$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private interrupts$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private spell_steals$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private dispels$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private threat_wipes$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private summons$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private melee_damage$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private spell_damage$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private heal$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private threat$: BehaviorSubject<Array<Event>> = new BehaviorSubject([]);
    private instance_meta$: BehaviorSubject<InstanceViewerMeta> = new BehaviorSubject(undefined);

    constructor(private apiService: APIService) {
    }

    private load_instance_data(event_type: number, on_success: any): void {
        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString())
                .replace(":event_type", event_type.toString()),
            on_success, () => {
            }
        );
    }

    private load_instance_meta(on_success: any): void {
        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_META_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString()),
            on_success, () => {
            }
        );
    }

    public set instance_meta_id(instance_meta_id: number) {
        this.instance_meta_id$ = instance_meta_id;
    }

    public get spell_casts(): Observable<Array<Event>> {
        this.load_instance_data(0, events => this.spell_casts$.next(events));
        return this.spell_casts$.asObservable();
    }

    public get deaths(): Observable<Array<Event>> {
        this.load_instance_data(1, events => this.deaths$.next(events));
        return this.deaths$.asObservable();
    }

    public get combat_states(): Observable<Array<Event>> {
        this.load_instance_data(2, events => this.combat_states$.next(events));
        return this.combat_states$.asObservable();
    }

    public get loot(): Observable<Array<Event>> {
        this.load_instance_data(3, events => this.loot$.next(events));
        return this.loot$.asObservable();
    }

    public get positions(): Observable<Array<Event>> {
        this.load_instance_data(4, events => this.positions$.next(events));
        return this.positions$.asObservable();
    }

    public get powers(): Observable<Array<Event>> {
        this.load_instance_data(5, events => this.powers$.next(events));
        return this.powers$.asObservable();
    }

    public get aura_applications(): Observable<Array<Event>> {
        this.load_instance_data(6, events => this.aura_applications$.next(events));
        return this.aura_applications$.asObservable();
    }

    public get interrupts(): Observable<Array<Event>> {
        this.load_instance_data(7, events => this.interrupts$.next(events));
        return this.interrupts$.asObservable();
    }

    public get spell_steals(): Observable<Array<Event>> {
        this.load_instance_data(8, events => this.spell_steals$.next(events));
        return this.spell_steals$.asObservable();
    }

    public get dispels(): Observable<Array<Event>> {
        this.load_instance_data(9, events => this.dispels$.next(events));
        return this.dispels$.asObservable();
    }

    public get threat_wipes(): Observable<Array<Event>> {
        this.load_instance_data(10, events => this.threat_wipes$.next(events));
        return this.threat_wipes$.asObservable();
    }

    public get summons(): Observable<Array<Event>> {
        this.load_instance_data(11, events => this.summons$.next(events));
        return this.summons$.asObservable();
    }

    public get melee_damage(): Observable<Array<Event>> {
        this.load_instance_data(12, events => this.melee_damage$.next(events));
        return this.melee_damage$.asObservable();
    }

    public get spell_damage(): Observable<Array<Event>> {
        this.load_instance_data(13, events => this.spell_damage$.next(events));
        return this.spell_damage$.asObservable();
    }

    public get heal(): Observable<Array<Event>> {
        this.load_instance_data(14, events => this.heal$.next(events));
        return this.heal$.asObservable();
    }

    public get threat(): Observable<Array<Event>> {
        this.load_instance_data(15, events => this.threat$.next(events));
        return this.threat$.asObservable();
    }

    public get meta(): Observable<InstanceViewerMeta> {
        this.load_instance_meta(meta => this.instance_meta$.next(meta));
        return this.instance_meta$.asObservable();
    }

}
