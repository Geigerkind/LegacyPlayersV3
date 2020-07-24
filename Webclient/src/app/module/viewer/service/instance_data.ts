import {Injectable} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, Subject} from "rxjs";
import {Event} from "../domain_value/event";

@Injectable({
    providedIn: "root",
})
export class InstanceDataService {
    private static INSTANCE_EXPORT_URL: string = "/instance/export/:instance_meta_id/:event_type";

    private instance_meta_id$: number;

    private spell_casts$: Subject<Array<Event>> = new Subject();
    private deaths$: Subject<Array<Event>> = new Subject();
    private combat_states$: Subject<Array<Event>> = new Subject();
    private loot$: Subject<Array<Event>> = new Subject();
    private positions$: Subject<Array<Event>> = new Subject();
    private powers$: Subject<Array<Event>> = new Subject();
    private aura_applications$: Subject<Array<Event>> = new Subject();
    private interrupts$: Subject<Array<Event>> = new Subject();
    private spell_steals$: Subject<Array<Event>> = new Subject();
    private dispels$: Subject<Array<Event>> = new Subject();
    private threat_wipes$: Subject<Array<Event>> = new Subject();
    private summons$: Subject<Array<Event>> = new Subject();
    private melee_damage$: Subject<Array<Event>> = new Subject();
    private spell_damage$: Subject<Array<Event>> = new Subject();
    private heal$: Subject<Array<Event>> = new Subject();
    private threat$: Subject<Array<Event>> = new Subject();

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

}
