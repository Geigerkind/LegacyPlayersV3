import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {SettingsService} from "src/app/service/settings";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";

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
    InstanceMeta,
    Participants
}

@Injectable({
    providedIn: "root",
})
export class InstanceDataService implements OnDestroy {
    private static INSTANCE_EXPORT_URL: string = "/instance/export/:instance_meta_id/:event_type/:last_event_id";
    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";
    private static INSTANCE_EXPORT_PARTICIPANTS_URL: string = "/instance/export/participants/:instance_meta_id";

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
    private participants$: BehaviorSubject<Array<InstanceViewerParticipants>>;

    private changed$: Subject<ChangedSubject> = new Subject();
    private registered_subjects: Array<[number, string, BehaviorSubject<Array<Event>>]> = [];
    private readonly updater: any;

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
        this.updater = setInterval(() => {
            for (const [event_type, storage_key, subject] of this.registered_subjects) {
                this.load_instance_data(event_type, (result: Array<Event>) => {
                    if (result.length > 0) {
                        let events: Array<Event> = subject.getValue();
                        events = events.concat(result);
                        subject.next(events);
                        this.changed$.next(event_type + 1);
                        this.settingsService.set_with_expiration(storage_key, events, 1);
                    }
                }, subject);
            }
        }, 60000);
    }

    ngOnDestroy(): void {
        clearInterval(this.updater);
    }

    private register_load_instance(event_type: number, storage_key: string, subject: BehaviorSubject<Array<Event>>): void {
        if (!this.registered_subjects.find(([i_event_type, i_storage_key, i_subject]) => i_event_type === event_type))
            this.registered_subjects.push([event_type, storage_key, subject]);
    }

    private load_instance_data(event_type: number, on_success: any, subject: BehaviorSubject<Array<Event>>): void {
        if (!this.instance_meta_id$)
            return;

        let last_event_id = 0;
        if (!!subject?.getValue() && subject.getValue().length > 0) {
            const value = subject.getValue();
            last_event_id = value[value.length - 1].id;
        }

        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString())
                .replace(":event_type", event_type.toString())
                .replace(":last_event_id", last_event_id.toString()),
            on_success, () => {
            }
        );
    }

    private load_instance_meta(on_success: any): void {
        if (!this.instance_meta_id$)
            return;

        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_META_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString()),
            on_success, () => {
            }
        );
    }

    private load_participants(on_success: any): void {
        if (!this.instance_meta_id$)
            return;

        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_PARTICIPANTS_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString()),
            on_success, () => {
            }
        );
    }

    public set instance_meta_id(instance_meta_id: number) {
        this.instance_meta_id$ = instance_meta_id;
    }

    public get spell_casts(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_casts$;
        this.spell_casts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_casts", 1, this.spell_casts$, [],
            (callback) => this.load_instance_data(0, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellCast);
            }, this.spell_casts$));
        if (register) this.register_load_instance(0, "instance_data_service_spell_casts", this.spell_casts$);
        return this.spell_casts$.asObservable();
    }

    public get deaths(): Observable<Array<Event>> {
        const register: boolean = !!this.deaths$;
        this.deaths$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_deaths", 1, this.deaths$, [],
            (callback) => this.load_instance_data(1, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Death);
            }, this.deaths$));
        if (register) this.register_load_instance(1, "instance_data_service_deaths", this.deaths$);
        return this.deaths$.asObservable();
    }

    public get combat_states(): Observable<Array<Event>> {
        const register: boolean = !!this.combat_states$;
        this.combat_states$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_combat_states", 1, this.combat_states$, [],
            (callback) => this.load_instance_data(2, events => {
                callback(events);
                this.changed$.next(ChangedSubject.CombatState);
            }, this.combat_states$));
        if (register) this.register_load_instance(2, "instance_data_service_combat_states", this.combat_states$);
        return this.combat_states$.asObservable();
    }

    public get loot(): Observable<Array<Event>> {
        const register: boolean = !!this.loot$;
        this.loot$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_loot", 1, this.loot$, [],
            (callback) => this.load_instance_data(3, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Loot);
            }, this.loot$));
        if (register) this.register_load_instance(3, "instance_data_service_loot", this.loot$);
        return this.loot$.asObservable();
    }

    public get positions(): Observable<Array<Event>> {
        const register: boolean = !!this.positions$;
        this.positions$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_positions", 1, this.positions$, [],
            (callback) => this.load_instance_data(4, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Position);
            }, this.positions$));
        if (register) this.register_load_instance(4, "instance_data_service_positions", this.positions$);
        return this.positions$.asObservable();
    }

    public get powers(): Observable<Array<Event>> {
        const register: boolean = !!this.powers$;
        this.powers$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_powers", 1, this.powers$, [],
            (callback) => this.load_instance_data(5, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Power);
            }, this.powers$));
        if (register) this.register_load_instance(5, "instance_data_service_powers", this.powers$);
        return this.powers$.asObservable();
    }

    public get aura_applications(): Observable<Array<Event>> {
        const register: boolean = !!this.aura_applications$;
        this.aura_applications$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_aura_applications", 1, this.aura_applications$, [],
            (callback) => this.load_instance_data(6, events => {
                callback(events);
                this.changed$.next(ChangedSubject.AuraApplication);
            }, this.aura_applications$));
        if (register) this.register_load_instance(6, "instance_data_service_aura_applications", this.aura_applications$);
        return this.aura_applications$.asObservable();
    }

    public get interrupts(): Observable<Array<Event>> {
        const register: boolean = !!this.interrupts$;
        this.interrupts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_interrupts", 1, this.interrupts$, [],
            (callback) => this.load_instance_data(7, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Interrupt);
            }, this.interrupts$));
        if (register) this.register_load_instance(7, "instance_data_service_interrupts", this.interrupts$);
        return this.interrupts$.asObservable();
    }

    public get spell_steals(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_steals$;
        this.spell_steals$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_steals", 1, this.spell_steals$, [],
            (callback) => this.load_instance_data(8, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellSteal);
            }, this.spell_casts$));
        if (register) this.register_load_instance(8, "instance_data_service_spell_steals", this.spell_steals$);
        return this.spell_steals$.asObservable();
    }

    public get dispels(): Observable<Array<Event>> {
        const register: boolean = !!this.dispels$;
        this.dispels$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_dispels", 1, this.dispels$, [],
            (callback) => this.load_instance_data(9, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Dispel);
            }, this.dispels$));
        if (register) this.register_load_instance(9, "instance_data_service_dispels", this.dispels$);
        return this.dispels$.asObservable();
    }

    public get threat_wipes(): Observable<Array<Event>> {
        const register: boolean = !!this.threat_wipes$;
        this.threat_wipes$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat_wipes", 1, this.threat_wipes$, [],
            (callback) => this.load_instance_data(10, events => {
                callback(events);
                this.changed$.next(ChangedSubject.ThreatWipe);
            }, this.threat_wipes$));
        if (register) this.register_load_instance(10, "instance_data_service_threat_wipes", this.threat_wipes$);
        return this.threat_wipes$.asObservable();
    }

    public get summons(): Observable<Array<Event>> {
        const register: boolean = !!this.summons$;
        this.summons$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_summons", 1, this.summons$, [],
            (callback) => this.load_instance_data(11, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Summon);
            }, this.summons$));
        if (register) this.register_load_instance(11, "instance_data_service_summons", this.summons$);
        return this.summons$.asObservable();
    }

    public get melee_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.melee_damage$;
        this.melee_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_melee_damage", 1, this.melee_damage$, [],
            (callback) => this.load_instance_data(12, events => {
                callback(events);
                this.changed$.next(ChangedSubject.MeleeDamage);
            }, this.melee_damage$));
        if (register) this.register_load_instance(12, "instance_data_service_melee_damage", this.melee_damage$);
        return this.melee_damage$.asObservable();
    }

    public get spell_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_damage$;
        this.spell_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_damage", 1, this.spell_damage$, [],
            (callback) => this.load_instance_data(13, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellDamage);
            }, this.spell_damage$));
        if (register) this.register_load_instance(13, "instance_data_service_spell_damage", this.spell_damage$);
        return this.spell_damage$.asObservable();
    }

    public get heal(): Observable<Array<Event>> {
        const register: boolean = !!this.heal$;
        this.heal$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_heal", 1, this.heal$, [],
            (callback) => this.load_instance_data(14, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Heal);
            }, this.heal$));
        if (register) this.register_load_instance(14, "instance_data_service_heal", this.heal$);
        return this.heal$.asObservable();
    }

    public get threat(): Observable<Array<Event>> {
        const register: boolean = !!this.threat$;
        this.threat$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat", 1, this.threat$, [],
            (callback) => this.load_instance_data(15, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Threat);
            }, this.threat$));
        if (register) this.register_load_instance(15, "instance_data_service_threat", this.threat$);
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

    public get participants(): Observable<Array<InstanceViewerParticipants>> {
        this.participants$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_participants", 1, this.participants$, [],
            (callback) => this.load_participants(participants => {
                callback(participants);
                this.changed$.next(ChangedSubject.Participants);
            }));
        return this.participants$.asObservable();
    }

    public get changed(): Observable<ChangedSubject> {
        return this.changed$.asObservable();
    }

}
