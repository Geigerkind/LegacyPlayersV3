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
import {map, take} from "rxjs/operators";
import {Interrupt} from "../domain_value/interrupt";
import {SpellSteal} from "../domain_value/spell_steal";
import {Dispel} from "../domain_value/dispel";

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
    Targets,
    Abilities,
    AttemptTotalDuration
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
    private abilities$: BehaviorSubject<Set<number>> = new BehaviorSubject(new Set());
    private ability_filter$: Set<number> = new Set();

    private attempt_total_duration$: BehaviorSubject<number> = new BehaviorSubject(1);

    private changed$: Subject<ChangedSubject> = new Subject();
    private registered_subjects: Array<[number, string, BehaviorSubject<Array<Event>>]> = [];
    private readonly updater: any;

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
        this.updater = setInterval(() => {
            this.load_instance_meta(meta => {
                this.instance_meta$.next(meta);
                this.changed$.next(ChangedSubject.InstanceMeta);
            });
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

    private extract_subjects(events: Array<Event>, target_extractor: (Event) => Unit, ability_extractor: (Event) => Array<number>): void {
        const current_sources = this.sources$.getValue();
        const current_targets = this.targets$.getValue();
        const current_abilities = this.abilities$.getValue();
        for (const event of events) {
            if (this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) === undefined)
                continue;

            if (!has_unit(current_sources, event.subject))
                current_sources.push(event.subject);

            if (!!target_extractor) {
                const target = target_extractor(event);
                if (!!target && !has_unit(current_targets, target))
                    current_targets.push(target);
            }

            if (!!ability_extractor) {
                const abilities = ability_extractor(event);
                for (const ability of abilities)
                    if ((!!ability || ability === 0) && !current_abilities.has(ability))
                        current_abilities.add(ability);
            }
        }
        this.sources$.next(current_sources);
        this.targets$.next(current_targets);
        this.abilities$.next(current_abilities);
        this.changed$.next(ChangedSubject.Sources);
        this.changed$.next(ChangedSubject.Targets);
        this.changed$.next(ChangedSubject.Abilities);
    }

    // Lets see how that performs :'D
    private fire_subjects_filter_changed(): void {
        if (!!this.spell_casts$) this.changed$.next(ChangedSubject.SpellCast);
        if (!!this.deaths$) this.changed$.next(ChangedSubject.Death);
        if (!!this.combat_states$) this.changed$.next(ChangedSubject.CombatState);
        if (!!this.loot$) this.changed$.next(ChangedSubject.Loot);
        if (!!this.positions$) this.changed$.next(ChangedSubject.Position);
        if (!!this.powers$) this.changed$.next(ChangedSubject.Power);
        if (!!this.aura_applications$) this.changed$.next(ChangedSubject.AuraApplication);
        if (!!this.interrupts$) this.changed$.next(ChangedSubject.Interrupt);
        if (!!this.spell_steals$) this.changed$.next(ChangedSubject.SpellSteal);
        if (!!this.dispels$) this.changed$.next(ChangedSubject.Dispel);
        if (!!this.threat_wipes$) this.changed$.next(ChangedSubject.ThreatWipe);
        if (!!this.summons$) this.changed$.next(ChangedSubject.Summon);
        if (!!this.melee_damage$) this.changed$.next(ChangedSubject.MeleeDamage);
        if (!!this.spell_damage$) this.changed$.next(ChangedSubject.SpellDamage);
        if (!!this.heal$) this.changed$.next(ChangedSubject.Heal);
        if (!!this.threat$) this.changed$.next(ChangedSubject.Threat);
    }

    public set attempt_intervals(intervals: Array<[number, number]>) {
        this.attempt_intervals$ = intervals;
        this.attempt_total_duration$.next(intervals.reduce((acc, [start, end]) => acc + end - start, 1));
        this.changed$.next(ChangedSubject.AttemptTotalDuration);

        // Update sources and targets
        this.sources$.next([]);
        this.targets$.next([]);
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

    public set source_filter(filtered_sources: Array<number>) {
        this.source_filter$ = filtered_sources;
        this.fire_subjects_filter_changed();
    }

    public set target_filter(filtered_targets: Array<number>) {
        this.target_filter$ = filtered_targets;
        this.fire_subjects_filter_changed();
    }

    public set ability_filter(filtered_abilities: Set<number>) {
        this.ability_filter$ = filtered_abilities;
        this.fire_subjects_filter_changed();
    }

    public set instance_meta_id(instance_meta_id: number) {
        this.instance_meta_id$ = instance_meta_id;
    }

    private apply_filter_to_events(subject: Observable<Array<Event>>, target_extraction: (Event) => Unit, ability_extraction: (Event) => Array<number>): Observable<Array<Event>> {
        return this.apply_ability_filter_to_events(
            this.apply_target_filter_to_events(
                this.apply_interval_and_source_filter_to_events(subject),
                target_extraction),
            ability_extraction);
    }

    private apply_interval_and_source_filter_to_events(subject: Observable<Array<Event>>): Observable<Array<Event>> {
        return subject.pipe(
            map(events => events.filter(event => this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) !== undefined)),
            map(events => events.filter(event => this.source_filter$.find(unit_id => unit_id === get_unit_id(event.subject)) !== undefined))
        );
    }

    private apply_target_filter_to_events(subject: Observable<Array<Event>>, target_extraction: (Event) => Unit): Observable<Array<Event>> {
        return subject.pipe(map(events => events.filter(event => get_unit_id(event.subject) === get_unit_id(target_extraction(event)) ||
            this.target_filter$.find(unit_id => unit_id === get_unit_id(target_extraction(event))) !== undefined)));
    }

    private apply_ability_filter_to_events(subject: Observable<Array<Event>>, ability_extraction: (Event) => Array<number>): Observable<Array<Event>> {
        return subject.pipe(map(events => events.filter(event => ability_extraction(event).every(ability => this.ability_filter$.has(ability)))));
    }

    public get spell_casts(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_casts$;
        const target_extraction = (event: Event) => ((event.event as any).SpellCast as SpellCast).victim;
        const ability_extraction = (event: Event) => [((event.event as any).SpellCast as SpellCast).spell_id];
        this.spell_casts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_casts", 1, this.spell_casts$, [],
            (callback) => this.load_instance_data(0, events => {
                this.subscriptions.push(this.spell_casts$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.SpellCast);

                // To solve dependency issues
                this.interrupts$?.next(this.interrupts$.getValue());
                this.spell_steals$?.next(this.spell_steals$.getValue());
                this.dispels$?.next(this.dispels$.getValue());
                this.spell_damage$?.next(this.spell_damage$.getValue());
                this.heal$?.next(this.heal$.getValue());
                this.threat$?.next(this.threat$.getValue());
            }, this.spell_casts$));
        if (register) this.register_load_instance(0, "instance_data_service_spell_casts", this.spell_casts$);
        return this.apply_filter_to_events(this.spell_casts$.asObservable(), target_extraction, ability_extraction);
    }

    public get deaths(): Observable<Array<Event>> {
        const register: boolean = !!this.deaths$;
        this.deaths$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_deaths", 1, this.deaths$, [],
            (callback) => this.load_instance_data(1, events => {
                this.subscriptions.push(this.deaths$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.Death);
            }, this.deaths$));
        if (register) this.register_load_instance(1, "instance_data_service_deaths", this.deaths$);
        return this.apply_interval_and_source_filter_to_events(this.deaths$.asObservable());
    }

    public get combat_states(): Observable<Array<Event>> {
        const register: boolean = !!this.combat_states$;
        this.combat_states$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_combat_states", 1, this.combat_states$, [],
            (callback) => this.load_instance_data(2, events => {
                this.subscriptions.push(this.combat_states$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.CombatState);
            }, this.combat_states$));
        if (register) this.register_load_instance(2, "instance_data_service_combat_states", this.combat_states$);
        return this.apply_interval_and_source_filter_to_events(this.combat_states$.asObservable());
    }

    public get loot(): Observable<Array<Event>> {
        const register: boolean = !!this.loot$;
        this.loot$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_loot", 1, this.loot$, [],
            (callback) => this.load_instance_data(3, events => {
                this.subscriptions.push(this.loot$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.Loot);
            }, this.loot$));
        if (register) this.register_load_instance(3, "instance_data_service_loot", this.loot$);
        return this.apply_interval_and_source_filter_to_events(this.loot$.asObservable());
    }

    public get positions(): Observable<Array<Event>> {
        const register: boolean = !!this.positions$;
        this.positions$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_positions", 1, this.positions$, [],
            (callback) => this.load_instance_data(4, events => {
                this.subscriptions.push(this.positions$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.Position);
            }, this.positions$));
        if (register) this.register_load_instance(4, "instance_data_service_positions", this.positions$);
        return this.apply_interval_and_source_filter_to_events(this.positions$.asObservable());
    }

    public get powers(): Observable<Array<Event>> {
        const register: boolean = !!this.powers$;
        this.powers$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_powers", 1, this.powers$, [],
            (callback) => this.load_instance_data(5, events => {
                this.subscriptions.push(this.powers$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.Power);
            }, this.powers$));
        if (register) this.register_load_instance(5, "instance_data_service_powers", this.powers$);
        return this.apply_interval_and_source_filter_to_events(this.powers$.asObservable());
    }

    public get aura_applications(): Observable<Array<Event>> {
        const register: boolean = !!this.aura_applications$;
        const target_extraction = (event: Event) => ((event.event as any).AuraApplication as AuraApplication).caster;
        const ability_extraction = (event: Event) => [((event.event as any).AuraApplication as AuraApplication).spell_id];
        this.aura_applications$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_aura_applications", 1, this.aura_applications$, [],
            (callback) => this.load_instance_data(6, events => {
                this.subscriptions.push(this.aura_applications$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.AuraApplication);

                // To solve dependency issues
                this.interrupts$?.next(this.interrupts$.getValue());
                this.spell_steals$?.next(this.spell_steals$.getValue());
                this.dispels$?.next(this.dispels$.getValue());
            }, this.aura_applications$));
        if (register) this.register_load_instance(6, "instance_data_service_aura_applications", this.aura_applications$);
        return this.apply_filter_to_events(this.aura_applications$.asObservable(), target_extraction, ability_extraction);
    }

    public get interrupts(): Observable<Array<Event>> {
        const register: boolean = !!this.interrupts$;
        const target_extraction = (event: Event) => {
            const interrupt = (event.event as any).Interrupt as Interrupt;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === interrupt.cause_event_id);
            if (!!spell_cast_event)
                return (spell_cast_event.event as any).SpellCast.victim;
            const aura_applications = this.aura_applications$?.getValue();
            const aura_application_event = aura_applications?.find(aura_app_event => aura_app_event.id === interrupt.cause_event_id);
            return (aura_application_event?.event as any)?.AuraApplication?.caster;
        };
        const ability_extraction = (event: Event) => {
            const interrupt = (event.event as any).Interrupt as Interrupt;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === interrupt.cause_event_id);
            if (!!spell_cast_event)
                return [(spell_cast_event.event as any).SpellCast.spell_id, interrupt.interrupted_spell_id];
            const aura_applications = this.aura_applications$?.getValue();
            const aura_application_event = aura_applications?.find(aura_app_event => aura_app_event.id === interrupt.cause_event_id);
            return [(aura_application_event?.event as any)?.AuraApplication?.spell_id, interrupt.interrupted_spell_id];
        };
        this.spell_casts.pipe(take(1));
        this.aura_applications.pipe(take(1));
        this.interrupts$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_interrupts", 1, this.interrupts$, [],
            (callback) => this.load_instance_data(7, events => {
                this.subscriptions.push(this.interrupts$.subscribe(i_events => this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.Interrupt);
            }, this.interrupts$));
        if (register) this.register_load_instance(7, "instance_data_service_interrupts", this.interrupts$);
        return this.apply_filter_to_events(this.interrupts$.asObservable(), target_extraction, ability_extraction);
    }

    public get spell_steals(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_steals$;
        const target_extraction = (event: Event) => {
            const spell_steal = (event.event as any).SpellSteal as SpellSteal;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === spell_steal.cause_event_id);
            return (spell_cast_event?.event as any).SpellCast.victim;
        };
        const ability_extraction = (event: Event) => {
            const spell_steal = (event.event as any).SpellSteal as SpellSteal;
            const spell_casts = this.spell_casts$?.getValue();
            const aura_applications = this.aura_applications$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === spell_steal.cause_event_id);
            const aura_application_event = aura_applications?.find(aura_app_event => aura_app_event.id === spell_steal.target_event_id);
            return [(aura_application_event?.event as any)?.AuraApplication?.spell_id, (spell_cast_event?.event as any)?.SpellCast?.spell_id];
        };
        this.spell_casts.pipe(take(1));
        this.aura_applications.pipe(take(1));
        this.spell_steals$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_steals", 1, this.spell_steals$, [],
            (callback) => this.load_instance_data(8, events => {
                this.subscriptions.push(this.spell_steals$.subscribe(i_events => this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.SpellSteal);
            }, this.spell_casts$));
        if (register) this.register_load_instance(8, "instance_data_service_spell_steals", this.spell_steals$);
        return this.apply_filter_to_events(this.spell_steals$.asObservable(), target_extraction, ability_extraction);
    }

    public get dispels(): Observable<Array<Event>> {
        const register: boolean = !!this.dispels$;
        const target_extraction = (event: Event) => {
            const dispel = (event.event as any).Dispel as Dispel;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === dispel.cause_event_id);
            return (spell_cast_event?.event as any)?.SpellCast?.victim;
        };
        const ability_extraction = (event: Event) => {
            const dispel = (event.event as any).Dispel as Dispel;
            const spell_casts = this.spell_casts$?.getValue();
            const aura_applications = this.aura_applications$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === dispel.cause_event_id);
            const result = [(spell_cast_event?.event as any)?.SpellCast?.spell_id];
            dispel.target_event_ids.forEach(target_event_id =>
                result.push((aura_applications?.find(aura_app_event => aura_app_event.id === target_event_id).event as any).AuraApplication.spell_id));
            return result;
        };
        this.spell_casts.pipe(take(1));
        this.aura_applications.pipe(take(1));
        this.dispels$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_dispels", 1, this.dispels$, [],
            (callback) => this.load_instance_data(9, events => {
                this.subscriptions.push(this.dispels$.subscribe(i_events => this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.Dispel);
            }, this.dispels$));
        if (register) this.register_load_instance(9, "instance_data_service_dispels", this.dispels$);
        return this.apply_filter_to_events(this.dispels$.asObservable(), target_extraction, ability_extraction);
    }

    public get threat_wipes(): Observable<Array<Event>> {
        const register: boolean = !!this.threat_wipes$;
        this.threat_wipes$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat_wipes", 1, this.threat_wipes$, [],
            (callback) => this.load_instance_data(10, events => {
                this.subscriptions.push(this.threat_wipes$.subscribe(i_events => this.extract_subjects(i_events, undefined, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.ThreatWipe);
            }, this.threat_wipes$));
        if (register) this.register_load_instance(10, "instance_data_service_threat_wipes", this.threat_wipes$);
        return this.apply_interval_and_source_filter_to_events(this.threat_wipes$.asObservable());
    }

    public get summons(): Observable<Array<Event>> {
        const register: boolean = !!this.summons$;
        const target_extraction = (event: Event) => ((event.event as any).Summon as Summon).summoned;
        this.summons$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_summons", 1, this.summons$, [],
            (callback) => this.load_instance_data(11, events => {
                this.subscriptions.push(this.summons$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, undefined)));
                callback(events);
                this.changed$.next(ChangedSubject.Summon);
            }, this.summons$));
        if (register) this.register_load_instance(11, "instance_data_service_summons", this.summons$);
        return this.apply_target_filter_to_events(this.apply_interval_and_source_filter_to_events(this.summons$), target_extraction);
    }

    public get melee_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.melee_damage$;
        const target_extraction = (event: Event) => ((event.event as any).MeleeDamage as MeleeDamage).victim;
        const ability_extraction = (event: Event) => [0];
        this.melee_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_melee_damage", 1, this.melee_damage$, [],
            (callback) => this.load_instance_data(12, events => {
                this.subscriptions.push(this.melee_damage$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.MeleeDamage);

                // To solve dependency issues
                this.threat$?.next(this.threat$.getValue());
            }, this.melee_damage$));
        if (register) this.register_load_instance(12, "instance_data_service_melee_damage", this.melee_damage$);
        return this.apply_filter_to_events(this.melee_damage$.asObservable(), target_extraction, ability_extraction);
    }

    public get spell_damage(): Observable<Array<Event>> {
        const register: boolean = !!this.spell_damage$;
        const target_extraction = (event: Event) => ((event.event as any).SpellDamage as SpellDamage).damage.victim;
        const ability_extraction = (event: Event) => {
            const spell_damage = (event.event as any).SpellDamage as SpellDamage;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === spell_damage.spell_cast_id);
            return [(spell_cast_event?.event as any)?.SpellCast?.spell_id];
        };
        this.spell_casts.pipe(take(1));
        this.spell_damage$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_spell_damage", 1, this.spell_damage$, [],
            (callback) => this.load_instance_data(13, events => {
                this.subscriptions.push(this.spell_damage$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.SpellDamage);
            }, this.spell_damage$));
        if (register) this.register_load_instance(13, "instance_data_service_spell_damage", this.spell_damage$);
        return this.apply_filter_to_events(this.spell_damage$.asObservable(), target_extraction, ability_extraction);
    }

    public get heal(): Observable<Array<Event>> {
        const register: boolean = !!this.heal$;
        const target_extraction = (event: Event) => ((event.event as any).Heal as Heal).heal.target;
        const ability_extraction = (event: Event) => {
            const heal = (event.event as any).Heal as Heal;
            const spell_casts = this.spell_casts$?.getValue();
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === heal.spell_cast_id);
            return [(spell_cast_event?.event as any)?.SpellCast?.spell_id];
        };
        this.spell_casts.pipe(take(1));
        this.heal$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_heal", 1, this.heal$, [],
            (callback) => this.load_instance_data(14, events => {
                this.subscriptions.push(this.heal$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.Heal);
            }, this.heal$));
        if (register) this.register_load_instance(14, "instance_data_service_heal", this.heal$);

        return this.apply_filter_to_events(this.heal$.asObservable(), target_extraction, ability_extraction);
    }

    public get threat(): Observable<Array<Event>> {
        const register: boolean = !!this.threat$;
        const target_extraction = (event: Event) => ((event.event as any).Threat as Threat).threat.threatened;
        const ability_extraction = (event: Event) => {
            const threat = (event.event as any).Threat as Threat;
            const spell_casts = this.spell_casts$?.getValue(); // TODO: We need to make sure that it exists
            const spell_cast_event = spell_casts?.find(cast_event => cast_event.id === threat.cause_event_id);
            if (!!spell_cast_event)
                return [(spell_cast_event.event as any).SpellCast.spell_id];
            const melee_damage = this.melee_damage$?.getValue(); // TODO: We need to make sure that it exists
            const melee_damage_event = melee_damage?.find(dmg_event => dmg_event.id === threat.cause_event_id);
            if (!!melee_damage_event)
                return [0];
            return [];
        };
        this.spell_casts.pipe(take(1));
        this.melee_damage.pipe(take(1));
        this.threat$ = this.settingsService.init_or_load_behavior_subject("instance_data_service_threat", 1, this.threat$, [],
            (callback) => this.load_instance_data(15, events => {
                this.subscriptions.push(this.threat$.subscribe(i_events =>
                    this.extract_subjects(i_events, target_extraction, ability_extraction)));
                callback(events);
                this.changed$.next(ChangedSubject.Threat);
            }, this.threat$));
        if (register) this.register_load_instance(15, "instance_data_service_threat", this.threat$);

        return this.apply_filter_to_events(this.threat$.asObservable(), target_extraction, ability_extraction);
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

    public get abilities(): Observable<Set<number>> {
        return this.abilities$.asObservable();
    }

    public get attempt_total_duration(): Observable<number> {
        return this.attempt_total_duration$.asObservable();
    }

}
