import {Injectable} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {init_behaviour_subject} from "../../../stdlib/init_behavior_subject";
import {publishReplay, refCount} from "rxjs/operators";

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

    constructor(private apiService: APIService) {
    }

    private trigger_next<T>(what_changed: number, subject: BehaviorSubject<T>, value: T): void {
        subject.next(value);
        this.changed$.next(what_changed);
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
        this.spell_casts$ = init_behaviour_subject(this.spell_casts$, [], (subject) => {
            this.load_instance_data(0, events => this.trigger_next(ChangedSubject.SpellCast, subject, events));
        });
        return this.spell_casts$.asObservable();
    }

    public get deaths(): Observable<Array<Event>> {
        this.deaths$ = init_behaviour_subject(this.deaths$, [], (subject) => {
            this.load_instance_data(1, events => this.trigger_next(ChangedSubject.Death, subject, events));
        });
        return this.deaths$.asObservable();
    }

    public get combat_states(): Observable<Array<Event>> {
        this.combat_states$ = init_behaviour_subject(this.combat_states$, [], (subject) => {
            this.load_instance_data(2, events => this.trigger_next(ChangedSubject.CombatState, subject, events));
        });
        return this.combat_states$.asObservable();
    }

    public get loot(): Observable<Array<Event>> {
        this.loot$ = init_behaviour_subject(this.loot$, [], (subject) => {
            this.load_instance_data(3, events => this.trigger_next(ChangedSubject.Loot, subject, events));
        });
        return this.loot$.asObservable();
    }

    public get positions(): Observable<Array<Event>> {
        this.positions$ = init_behaviour_subject(this.positions$, [], (subject) => {
            this.load_instance_data(4, events => this.trigger_next(ChangedSubject.Position, subject, events));
        });
        return this.positions$.asObservable();
    }

    public get powers(): Observable<Array<Event>> {
        this.powers$ = init_behaviour_subject(this.powers$, [], (subject) => {
            this.load_instance_data(5, events => this.trigger_next(ChangedSubject.Power, subject, events));
        });
        return this.powers$.asObservable();
    }

    public get aura_applications(): Observable<Array<Event>> {
        this.aura_applications$ = init_behaviour_subject(this.aura_applications$, [], (subject) => {
            this.load_instance_data(6, events => this.trigger_next(ChangedSubject.AuraApplication, subject, events));
        });
        return this.aura_applications$.asObservable();
    }

    public get interrupts(): Observable<Array<Event>> {
        this.interrupts$ = init_behaviour_subject(this.interrupts$, [], (subject) => {
            this.load_instance_data(7, events => this.trigger_next(ChangedSubject.Interrupt, subject, events));
        });
        return this.interrupts$.asObservable();
    }

    public get spell_steals(): Observable<Array<Event>> {
        this.spell_steals$ = init_behaviour_subject(this.spell_steals$, [], (subject) => {
            this.load_instance_data(8, events => this.trigger_next(ChangedSubject.SpellSteal, subject, events));
        });
        return this.spell_steals$.asObservable();
    }

    public get dispels(): Observable<Array<Event>> {
        this.dispels$ = init_behaviour_subject(this.dispels$, [], (subject) => {
            this.load_instance_data(9, events => this.trigger_next(ChangedSubject.Dispel, subject, events));
        });
        return this.dispels$.asObservable();
    }

    public get threat_wipes(): Observable<Array<Event>> {
        this.threat_wipes$ = init_behaviour_subject(this.threat_wipes$, [], (subject) => {
            this.load_instance_data(10, events => this.trigger_next(ChangedSubject.ThreatWipe, subject, events));
        });
        return this.threat_wipes$.asObservable();
    }

    public get summons(): Observable<Array<Event>> {
        this.summons$ = init_behaviour_subject(this.summons$, [], (subject) => {
            this.load_instance_data(11, events => this.trigger_next(ChangedSubject.Summon, subject, events));
        });
        return this.summons$.asObservable();
    }

    public get melee_damage(): Observable<Array<Event>> {
        this.melee_damage$ = init_behaviour_subject(this.melee_damage$, [], (subject) => {
            this.load_instance_data(12, events => this.trigger_next(ChangedSubject.MeleeDamage, subject, events));
        });
        return this.melee_damage$.asObservable();
    }

    public get spell_damage(): Observable<Array<Event>> {
        this.spell_damage$ = init_behaviour_subject(this.spell_damage$, [], (subject) => {
            this.load_instance_data(13, events => this.trigger_next(ChangedSubject.SpellDamage, subject, events));
        });
        return this.spell_damage$.asObservable();
    }

    public get heal(): Observable<Array<Event>> {
        this.heal$ = init_behaviour_subject(this.heal$, [], (subject) => {
            this.load_instance_data(14, events => this.trigger_next(ChangedSubject.Heal, subject, events));
        });
        return this.heal$.asObservable();
    }

    public get threat(): Observable<Array<Event>> {
        this.threat$ = init_behaviour_subject(this.threat$, [], (subject) => {
            this.load_instance_data(15, events => this.trigger_next(ChangedSubject.Threat, subject, events));
        });
        return this.threat$.asObservable();
    }

    public get meta(): Observable<InstanceViewerMeta> {
        this.instance_meta$ = init_behaviour_subject(this.instance_meta$, undefined, (subject) => {
            this.load_instance_meta(meta => this.trigger_next(ChangedSubject.InstanceMeta, subject, meta));
        });
        return this.instance_meta$.asObservable();
    }

    public get changed(): Observable<ChangedSubject> {
        return this.changed$.asObservable();
    }

}
