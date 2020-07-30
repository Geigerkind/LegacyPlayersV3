import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject, Subscription} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {SettingsService} from "src/app/service/settings";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";
import {InstanceViewerAttempt} from "../domain_value/instance_viewer_attempt";
import {get_unit_id, has_unit, Unit} from "../domain_value/unit";
import {SpellCast} from "../domain_value/spell_cast";
import {AuraApplication} from "../domain_value/aura_application";
import {MeleeDamage} from "../domain_value/melee_damage";
import {SpellDamage} from "../domain_value/spell_damage";
import {Heal} from "../domain_value/heal";
import {Threat} from "../domain_value/threat";
import {Summon} from "../domain_value/summon";
import {map} from "rxjs/operators";

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
    Participants,
    Attempts,
    Sources,
    Targets
}

@Injectable({
    providedIn: "root",
})
export class InstanceDataService implements OnDestroy {
    private static INSTANCE_EXPORT_URL: string = "/instance/export/:instance_meta_id/:event_type/:last_event_id";
    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";
    private static INSTANCE_EXPORT_PARTICIPANTS_URL: string = "/instance/export/participants/:instance_meta_id";
    private static INSTANCE_EXPORT_ATTEMPTS_URL: string = "/instance/export/attempts/:instance_meta_id";

    private subscriptions: Array<Subscription> = [];

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
    private attempts$: BehaviorSubject<Array<InstanceViewerAttempt>>;

    private attempt_intervals$: Array<[number, number]> = [];
    private sources$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private source_filter$: Array<number> = [];
    private targets$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private target_filter$: Array<number> = [];

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
        for (const subscription of this.subscriptions)
            subscription.unsubscribe();
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

    private load_attempts(on_success: any): void {
        if (!this.instance_meta_id$)
            return;

        return this.apiService.get(
            InstanceDataService.INSTANCE_EXPORT_ATTEMPTS_URL
                .replace(":instance_meta_id", this.instance_meta_id$.toString()),
            on_success, () => {
            }
        );
    }

    private extract_sources_and_targets(events: Array<Event>, target_extractor: (Event) => Unit): void {
        const current_sources = this.sources$.getValue();
        const current_targets = this.targets$.getValue();
        for (const event of events) {
            if (this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) === undefined)
                continue;

            if (!has_unit(current_sources, event.subject))
                current_sources.push(event.subject);
            const target = target_extractor(event);
            if (!has_unit(current_targets, target))
                current_targets.push(target);
        }
        this.sources$.next(current_sources);
        this.targets$.next(current_sources);
        this.changed$.next(ChangedSubject.Sources);
        this.changed$.next(ChangedSubject.Targets);
    }

    private extract_sources(events: Array<Event>): void {
        const current_sources = this.sources$.getValue();
        for (const event of events)
            if (!has_unit(current_sources, event.subject))
                current_sources.push(event.subject);
        this.sources$.next(current_sources);
        this.changed$.next(ChangedSubject.Sources);
    }

    // Lets see how that performs :'D
    private fire_subjects_filter_changed(): void {
        this.spell_casts$?.next(this.spell_casts$.getValue());
        this.deaths$?.next(this.deaths$.getValue());
        this.combat_states$?.next(this.combat_states$.getValue());
        this.loot$?.next(this.loot$.getValue());
        this.positions$?.next(this.positions$.getValue());
        this.powers$?.next(this.powers$.getValue());
        this.aura_applications$?.next(this.aura_applications$.getValue());
        this.interrupts$?.next(this.interrupts$.getValue());
        this.spell_steals$?.next(this.spell_steals$.getValue());
        this.dispels$?.next(this.dispels$.getValue());
        this.threat_wipes$?.next(this.threat_wipes$.getValue());
        this.summons$?.next(this.summons$.getValue());
        this.melee_damage$?.next(this.melee_damage$.getValue());
        this.spell_damage$?.next(this.spell_damage$.getValue());
        this.heal$?.next(this.heal$.getValue());
        this.threat$?.next(this.threat$.getValue());
    }

    public set attempt_intervals(intervals: Array<[number, number]>) {
        this.attempt_intervals$ = intervals;
        this.sources$.next([]);
        this.targets$.next([]);
        this.fire_subjects_filter_changed();
    }

    public set source_filter(filtered_sources: Array<number>) {
        this.source_filter$ = filtered_sources;
        this.fire_subjects_filter_changed();
    }

    public set target_filter(filtered_targets: Array<number>) {
        this.target_filter$ = filtered_targets;
        this.fire_subjects_filter_changed();
    }

    public set instance_meta_id(instance_meta_id: number) {
        this.instance_meta_id$ = instance_meta_id;
    }

    private apply_filter_to_events(subject: Observable<Array<Event>>, target_extraction: (Event) => Unit): Observable<Array<Event>> {
        return this.apply_target_filter_to_events(this.apply_interval_and_source_filter_to_events(subject), target_extraction);
    }

    private apply_interval_and_source_filter_to_events(subject: Observable<Array<Event>>): Observable<Array<Event>> {
        return subject.pipe(
            map(events => events.filter(event => this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) !== undefined)),
            map(events => events.filter(event => this.source_filter$.find(unit_id => unit_id === get_unit_id(event.subject) !== undefined)))
        );
    }

    private apply_target_filter_to_events(subject: Observable<Array<Event>>, target_extraction: (Event) => Unit): Observable<Array<Event>> {
        return subject.pipe(map(events => events.filter(event => this.target_filter$.find(unit_id => unit_id === get_unit_id(target_extraction(event)) !== undefined))));
    }

    public get spell_casts(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_casts$;
        const target_extraction = (event: Event) => (event.event as SpellCast).victim;
        this.spell_casts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_casts", 1, this.spell_casts$, [],
            (callback) => this.load_instance_data(0, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellCast);
            }, this.spell_casts$));
        if (register) this.register_load_instance(0, "instance_data_service_spell_casts", this.spell_casts$);

        this.subscriptions.push(this.spell_casts$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.spell_casts$.asObservable(), target_extraction);
    }

    public get deaths(): Observable<Array<Event>> {
        const register: boolean = !!this.deaths$;
        this.deaths$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_deaths", 1, this.deaths$, [],
            (callback) => this.load_instance_data(1, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Death);
            }, this.deaths$));
        if (register) this.register_load_instance(1, "instance_data_service_deaths", this.deaths$);
        this.subscriptions.push(this.deaths$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.deaths$.asObservable());
    }

    public get combat_states(): Observable<Array<Event>> {
        const register: boolean = !!this.combat_states$;
        this.combat_states$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_combat_states", 1, this.combat_states$, [],
            (callback) => this.load_instance_data(2, events => {
                callback(events);
                this.changed$.next(ChangedSubject.CombatState);
            }, this.combat_states$));
        if (register) this.register_load_instance(2, "instance_data_service_combat_states", this.combat_states$);
        this.subscriptions.push(this.combat_states$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.combat_states$.asObservable());
    }

    public get loot(): Observable<Array<Event>> {
        const register: boolean = !!this.loot$;
        this.loot$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_loot", 1, this.loot$, [],
            (callback) => this.load_instance_data(3, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Loot);
            }, this.loot$));
        if (register) this.register_load_instance(3, "instance_data_service_loot", this.loot$);
        this.subscriptions.push(this.loot$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.loot$.asObservable());
    }

    public get positions(): Observable<Array<Event>> {
        const register: boolean = !!this.positions$;
        this.positions$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_positions", 1, this.positions$, [],
            (callback) => this.load_instance_data(4, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Position);
            }, this.positions$));
        if (register) this.register_load_instance(4, "instance_data_service_positions", this.positions$);
        this.subscriptions.push(this.positions$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.positions$.asObservable());
    }

    public get powers(): Observable<Array<Event>> {
        const register: boolean = !!this.powers$;
        this.powers$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_powers", 1, this.powers$, [],
            (callback) => this.load_instance_data(5, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Power);
            }, this.powers$));
        if (register) this.register_load_instance(5, "instance_data_service_powers", this.powers$);
        this.subscriptions.push(this.powers$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.powers$.asObservable());
    }

    public get aura_applications(): Observable<Array<Event>> {
        const register: boolean = !!this.aura_applications$;
        const target_extraction = (event: Event) => (event.event as AuraApplication).caster;
        this.aura_applications$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_aura_applications", 1, this.aura_applications$, [],
            (callback) => this.load_instance_data(6, events => {
                callback(events);
                this.changed$.next(ChangedSubject.AuraApplication);
            }, this.aura_applications$));
        if (register) this.register_load_instance(6, "instance_data_service_aura_applications", this.aura_applications$);

        this.subscriptions.push(this.aura_applications$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.aura_applications$.asObservable(), target_extraction);
    }

    // TODO: These events need to have more sophisticated filter, looking up the events they refer to
    public get interrupts(): Observable<Array<Event>> {
        const register: boolean = !!this.interrupts$;
        this.interrupts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_interrupts", 1, this.interrupts$, [],
            (callback) => this.load_instance_data(7, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Interrupt);
            }, this.interrupts$));
        if (register) this.register_load_instance(7, "instance_data_service_interrupts", this.interrupts$);
        this.subscriptions.push(this.interrupts$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.interrupts$.asObservable());
    }

    // TODO: These events need to have more sophisticated filter, looking up the events they refer to
    public get spell_steals(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_steals$;
        this.spell_steals$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_steals", 1, this.spell_steals$, [],
            (callback) => this.load_instance_data(8, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellSteal);
            }, this.spell_casts$));
        if (register) this.register_load_instance(8, "instance_data_service_spell_steals", this.spell_steals$);
        this.subscriptions.push(this.spell_steals$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.spell_steals$.asObservable());
    }

    // TODO: These events need to have more sophisticated filter, looking up the events they refer to
    public get dispels(): Observable<Array<Event>> {
        const register: boolean = !!this.dispels$;
        this.dispels$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_dispels", 1, this.dispels$, [],
            (callback) => this.load_instance_data(9, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Dispel);
            }, this.dispels$));
        if (register) this.register_load_instance(9, "instance_data_service_dispels", this.dispels$);
        this.subscriptions.push(this.dispels$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.dispels$.asObservable());
    }

    public get threat_wipes(): Observable<Array<Event>> {
        const register: boolean = !!this.threat_wipes$;
        this.threat_wipes$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat_wipes", 1, this.threat_wipes$, [],
            (callback) => this.load_instance_data(10, events => {
                callback(events);
                this.changed$.next(ChangedSubject.ThreatWipe);
            }, this.threat_wipes$));
        if (register) this.register_load_instance(10, "instance_data_service_threat_wipes", this.threat_wipes$);
        this.subscriptions.push(this.threat_wipes$.subscribe(this.extract_sources));
        return this.apply_interval_and_source_filter_to_events(this.threat_wipes$.asObservable());
    }

    public get summons(): Observable<Array<Event>> {
        const register: boolean = !!this.summons$;
        const target_extraction = (event: Event) => (event.event as Summon).summoned;
        this.summons$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_summons", 1, this.summons$, [],
            (callback) => this.load_instance_data(11, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Summon);
            }, this.summons$));
        if (register) this.register_load_instance(11, "instance_data_service_summons", this.summons$);

        this.subscriptions.push(this.melee_damage$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.summons$.asObservable(), target_extraction);
    }

    public get melee_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.melee_damage$;
        const target_extraction = (event: Event) => (event.event as MeleeDamage).victim;
        this.melee_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_melee_damage", 1, this.melee_damage$, [],
            (callback) => this.load_instance_data(12, events => {
                callback(events);
                this.changed$.next(ChangedSubject.MeleeDamage);
            }, this.melee_damage$));
        if (register) this.register_load_instance(12, "instance_data_service_melee_damage", this.melee_damage$);

        this.subscriptions.push(this.melee_damage$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.melee_damage$.asObservable(), target_extraction);
    }

    public get spell_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_damage$;
        const target_extraction = (event: Event) => (event.event as SpellDamage).damage.victim;
        this.spell_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_damage", 1, this.spell_damage$, [],
            (callback) => this.load_instance_data(13, events => {
                callback(events);
                this.changed$.next(ChangedSubject.SpellDamage);
            }, this.spell_damage$));
        if (register) this.register_load_instance(13, "instance_data_service_spell_damage", this.spell_damage$);

        this.subscriptions.push(this.spell_damage$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.spell_damage$.asObservable(), target_extraction);
    }

    public get heal(): Observable<Array<Event>> {
        const register: boolean = !!this.heal$;
        const target_extraction = (event: Event) => (event.event as Heal).heal.target;
        this.heal$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_heal", 1, this.heal$, [],
            (callback) => this.load_instance_data(14, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Heal);
            }, this.heal$));
        if (register) this.register_load_instance(14, "instance_data_service_heal", this.heal$);

        this.subscriptions.push(this.heal$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.heal$.asObservable(), target_extraction);
    }

    public get threat(): Observable<Array<Event>> {
        const register: boolean = !!this.threat$;
        const target_extraction = (event: Event) => (event.event as Threat).threat.threatened;
        this.threat$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat", 1, this.threat$, [],
            (callback) => this.load_instance_data(15, events => {
                callback(events);
                this.changed$.next(ChangedSubject.Threat);
            }, this.threat$));
        if (register) this.register_load_instance(15, "instance_data_service_threat", this.threat$);

        this.subscriptions.push(this.threat$.subscribe(events =>
            this.extract_sources_and_targets(events, target_extraction)));

        return this.apply_filter_to_events(this.threat$.asObservable(), target_extraction);
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

    public get attempts(): Observable<Array<InstanceViewerAttempt>> {
        this.attempts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_attempts", 1, this.attempts$, [],
            (callback) => this.load_attempts(attempts => {
                callback(attempts);
                this.changed$.next(ChangedSubject.Attempts);
            }));
        return this.attempts$.asObservable();
    }

    public get changed(): Observable<ChangedSubject> {
        return this.changed$.asObservable();
    }

    public get sources(): Observable<Array<Unit>> {
        return this.sources$.asObservable();
    }

    public get targets(): Observable<Array<Unit>> {
        return this.targets$.asObservable();
    }

}
