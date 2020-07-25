import {Injectable} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {init_behavior_subject} from "../../../stdlib/init_behavior_subject";
import {SettingsService} from "src/app/service/settings";

export enum ChangedSubject {
    SpellCast = 1,
    Death = 2,
    CombatState,
    Loot,
    Position,
    Power,
    AuraApplication,
    Interrupt,
    SpellSteal,
    Dispel,
    ThreatWipe,
    Summon,
    MeleeDamage,
    SpellDamage,
    Heal,
    Threat,
    InstanceMeta
}

@Injectable({
    providedIn: "root",
})
export class InstanceDataService {
    private static INSTANCE_EXPORT_URL: string = "/instance/export/:instance_meta_id/:event_type";
    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";

    private instance_meta_id$: number;

    private spell_casts$: BehaviorSubject<Array<Event>>;
    private deaths$: BehaviorSubject<Array<Event>>;
    private combat_states$: BehaviorSubject<Array<Event>>;
    private loot$: BehaviorSubject<Array<Event>>;
    private positions$: BehaviorSubject<Array<Event>>;
    private powers$: BehaviorSubject<Array<Event>>;
    private aura_applications$: BehaviorSubject<Array<Event>>;
    private interrupts$: BehaviorSubject<Array<Event>>;
    private spell_steals$: BehaviorSubject<Array<Event>>;
    private dispels$: BehaviorSubject<Array<Event>>;
    private threat_wipes$: BehaviorSubject<Array<Event>>;
    private summons$: BehaviorSubject<Array<Event>>;
    private melee_damage$: BehaviorSubject<Array<Event>>;
    private spell_damage$: BehaviorSubject<Array<Event>>;
    private heal$: BehaviorSubject<Array<Event>>;
    private threat$: BehaviorSubject<Array<Event>>;
    private instance_meta$: BehaviorSubject<InstanceViewerMeta>;

    private changed$: Subject<ChangedSubject> = new Subject();

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
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
        this.spell_casts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_casts", 1, this.spell_casts$, [],
            (callback) => this.load_instance_data(0, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellCast);
            }));
        return this.spell_casts$.asObservable();
    }

    public get deaths(): Observable<Array<Event>> {
        this.deaths$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_deaths", 1, this.deaths$, [],
            (callback) => this.load_instance_data(1, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Death);
            }));
        return this.deaths$.asObservable();
    }

    public get combat_states(): Observable<Array<Event>> {
        this.combat_states$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_combat_states", 1, this.combat_states$, [],
            (callback) => this.load_instance_data(2, events => {
                callback(events);
                this.changed$.next(ChangedSubject.CombatState);
            }));
        return this.combat_states$.asObservable();
    }

    public get loot(): Observable<Array<Event>> {
        this.loot$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_loot", 1, this.loot$, [],
            (callback) => this.load_instance_data(3, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Loot);
            }));
        return this.loot$.asObservable();
    }

    public get positions(): Observable<Array<Event>> {
        this.positions$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_positions", 1, this.positions$, [],
            (callback) => this.load_instance_data(4, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Position);
            }));
        return this.positions$.asObservable();
    }

    public get powers(): Observable<Array<Event>> {
        this.powers$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_powers", 1, this.powers$, [],
            (callback) => this.load_instance_data(5, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Power);
            }));
        return this.powers$.asObservable();
    }

    public get aura_applications(): Observable<Array<Event>> {
        this.aura_applications$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_aura_applications", 1, this.aura_applications$, [],
            (callback) => this.load_instance_data(6, events => {
                callback(events);
                this.changed$.next(ChangedSubject.AuraApplication);
            }));
        return this.aura_applications$.asObservable();
    }

    public get interrupts(): Observable<Array<Event>> {
        this.interrupts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_interrupts", 1, this.interrupts$, [],
            (callback) => this.load_instance_data(7, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Interrupt);
            }));
        return this.interrupts$.asObservable();
    }

    public get spell_steals(): Observable<Array<Event>> {
        this.spell_steals$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_steals", 1, this.spell_steals$, [],
            (callback) => this.load_instance_data(8, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellSteal);
            }));
        return this.spell_steals$.asObservable();
    }

    public get dispels(): Observable<Array<Event>> {
        this.dispels$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_dispels", 1, this.dispels$, [],
            (callback) => this.load_instance_data(9, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Dispel);
            }));
        return this.dispels$.asObservable();
    }

    public get threat_wipes(): Observable<Array<Event>> {
        this.threat_wipes$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat_wipes", 1, this.threat_wipes$, [],
            (callback) => this.load_instance_data(10, events => {
                callback(events);
                this.changed$.next(ChangedSubject.ThreatWipe);
            }));
        return this.threat_wipes$.asObservable();
    }

    public get summons(): Observable<Array<Event>> {
        this.summons$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_summons", 1, this.summons$, [],
            (callback) => this.load_instance_data(11, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Summon);
            }));
        return this.summons$.asObservable();
    }

    public get melee_damage(): Observable<Array<Event>> {
        this.melee_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_melee_damage", 1, this.melee_damage$, [],
            (callback) => this.load_instance_data(12, events => {
                callback(events);
                this.changed$.next(ChangedSubject.MeleeDamage);
            }));
        return this.melee_damage$.asObservable();
    }

    public get spell_damage(): Observable<Array<Event>> {
        this.spell_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_damage", 1, this.spell_damage$, [],
            (callback) => this.load_instance_data(13, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellDamage);
            }));
        return this.spell_damage$.asObservable();
    }

    public get heal(): Observable<Array<Event>> {
        this.heal$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_heal", 1, this.heal$, [],
            (callback) => this.load_instance_data(14, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Heal);
            }));
        return this.heal$.asObservable();
    }

    public get threat(): Observable<Array<Event>> {
        this.threat$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat", 1, this.threat$, [],
            (callback) => this.load_instance_data(15, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Threat);
            }));
        return this.threat$.asObservable();
    }

    public get meta(): Observable<InstanceViewerMeta> {
        this.instance_meta$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_instance_meta", 1, this.instance_meta$, undefined,
            (callback) => this.load_instance_meta(meta => {
                callback(meta);
                this.changed$.next(ChangedSubject.InstanceMeta);
            }));
        return this.instance_meta$.asObservable();
    }

    public get changed(): Observable<ChangedSubject> {
        return this.changed$.asObservable();
    }

}
