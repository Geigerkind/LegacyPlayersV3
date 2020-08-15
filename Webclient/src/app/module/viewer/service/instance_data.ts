import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject, Subscription} from "rxjs";
import {Event} from "../domain_value/event";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";
import {InstanceViewerAttempt} from "../domain_value/instance_viewer_attempt";
import {get_unit_id, has_unit, Unit} from "../domain_value/unit";
import {map, take} from "rxjs/operators";
import {
    te_aura_application, te_heal, te_melee_damage,
    te_spell_cast,
    te_spell_cast_by_cause,
    te_spell_cast_or_aura_app, te_spell_damage, te_summon, te_threat
} from "../extractor/targets";
import {ce_dispel, ce_heal, ce_interrupt, ce_spell_damage, ce_spell_steal} from "../extractor/causes";
import {
    ae_aura_application,
    ae_dispel,
    ae_interrupt,
    ae_melee_damage,
    ae_spell_cast, ae_spell_cast_or_aura_application,
    ae_spell_steal, ae_threat
} from "../extractor/abilities";
import {se_aura_app_or_own, se_identity} from "../extractor/sources";

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

    // Map subjects are referenced by some other object
    // Threrefore they have an efficient ID lookup
    private spell_casts$: BehaviorSubject<Map<number, Event>>;
    private aura_applications$: BehaviorSubject<Map<number, Event>>;
    private melee_damage$: BehaviorSubject<Map<number, Event>>;
    private deaths$: BehaviorSubject<Array<Event>>;
    private combat_states$: BehaviorSubject<Array<Event>>;
    private loot$: BehaviorSubject<Array<Event>>;
    private positions$: BehaviorSubject<Array<Event>>;
    private powers$: BehaviorSubject<Array<Event>>;
    private interrupts$: BehaviorSubject<Array<Event>>;
    private spell_steals$: BehaviorSubject<Array<Event>>;
    private dispels$: BehaviorSubject<Array<Event>>;
    private threat_wipes$: BehaviorSubject<Array<Event>>;
    private summons$: BehaviorSubject<Array<Event>>;
    private spell_damage$: BehaviorSubject<Array<Event>>;
    private heal$: BehaviorSubject<Array<Event>>;
    private threat$: BehaviorSubject<Array<Event>>;
    private instance_meta$: BehaviorSubject<InstanceViewerMeta>;
    private participants$: BehaviorSubject<Array<InstanceViewerParticipants>>;
    private attempts$: BehaviorSubject<Array<InstanceViewerAttempt>>;

    private attempt_intervals$: Array<[number, number]> = [];
    private sources$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private source_filter$: Set<number> = new Set();
    private targets$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private target_filter$: Set<number> = new Set();
    private abilities$: BehaviorSubject<Set<number>> = new BehaviorSubject(new Set());
    private ability_filter$: Set<number> = new Set();

    private attempt_total_duration$: BehaviorSubject<number> = new BehaviorSubject(1);

    private changed$: Subject<ChangedSubject> = new Subject();
    private registered_subjects: Array<[number, BehaviorSubject<Array<Event> | Map<number, Event>>]> = [];
    private readonly updater: any;

    constructor(
        private apiService: APIService
    ) {
        this.updater = setInterval(() => {
            this.load_instance_meta(meta => {
                const current_meta = this.instance_meta$.getValue();
                if (current_meta.end_ts !== meta.end_ts) {
                    this.instance_meta$.next(meta);
                    this.changed$.next(ChangedSubject.InstanceMeta);
                }
            });
            this.load_attempts(attempts => {
                if (attempts.length > this.attempts$.getValue().length) {
                    this.attempts$.next(attempts);
                    this.changed$.next(ChangedSubject.Attempts);
                }
            });
            for (const [event_type, subject] of this.registered_subjects) {
                let subject_value = subject.getValue();
                const last_event_id = subject_value instanceof Map ?
                    [...subject_value.keys()].reduce((highest, id) => Math.max(highest, id), 0) :
                    (subject_value.length > 0 ? subject_value[subject_value.length - 1].id : 0);
                this.load_instance_data(event_type, (result: Array<Event>) => {
                    if (result.length > 0) {
                        if (subject_value instanceof Map)
                            result.forEach(event => (subject_value as Map<number, Event>).set(event.id, event));
                        else subject_value = subject_value.concat(result);
                        subject.next(subject_value);
                        this.changed$.next(event_type + 1);
                    }
                }, last_event_id);
            }
        }, 60000);
    }

    ngOnDestroy(): void {
        for (const subscription of this.subscriptions)
            subscription.unsubscribe();
        clearInterval(this.updater);
    }

    private register_load_instance(event_type: number, subject: BehaviorSubject<Array<Event> | Map<number, Event>>): void {
        if (!this.registered_subjects.find(([i_event_type, i_subject]) => i_event_type === event_type))
            this.registered_subjects.push([event_type, subject]);
    }

    private load_instance_data(event_type: number, on_success: any, last_event_id: number): void {
        if (!this.instance_meta_id$)
            return;

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

    private extract_subjects(events: Array<Event>, source_extractor: (Event) => Unit, target_extractor: (Event) => Unit, ability_extractor: (Event) => Array<number>): void {
        const current_sources = this.sources$.getValue();
        const current_targets = this.targets$.getValue();
        const current_abilities = this.abilities$.getValue();
        for (const event of events) {
            if (this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) === undefined)
                continue;

            const source = source_extractor(event);
            if (!has_unit(current_sources, source))
                current_sources.push(source);

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
        this.source_filter$ = new Set(filtered_sources);
        this.fire_subjects_filter_changed();
    }

    public set target_filter(filtered_targets: Array<number>) {
        this.target_filter$ = new Set(filtered_targets);
        this.fire_subjects_filter_changed();
    }

    public set ability_filter(filtered_abilities: Array<number>) {
        this.ability_filter$ = new Set(filtered_abilities);
        this.fire_subjects_filter_changed();
    }

    public set instance_meta_id(instance_meta_id: number) {
        this.instance_meta_id$ = instance_meta_id;
    }

    private apply_filter_to_events(subject: Observable<Array<Event>>, source_extraction: (Event) => Unit,
                                   target_extraction: (Event) => Unit, ability_extraction: (Event) => Array<number>, inverse_filter: boolean = false): Observable<Array<Event>> {
        return this.apply_ability_filter_to_events(
            this.apply_target_filter_to_events(
                this.apply_interval_and_source_filter_to_events(subject, source_extraction, inverse_filter),
                target_extraction, inverse_filter),
            ability_extraction);
    }

    private apply_filter_to_events_map(subject: Observable<Map<number, Event>>, source_extraction: (Event) => Unit,
                                       target_extraction: (Event) => Unit, ability_extraction: (Event) => Array<number>, inverse_filter: boolean = false): Observable<Map<number, Event>> {
        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        const filter_target = inverse_filter ? this.source_filter$ : this.target_filter$;
        return subject.pipe(
            map(events => [...events]),
            map(events => events.filter(([id, event]) => this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) !== undefined)),
            map(events => events.filter(([id, event]) => filter_source.has(get_unit_id(source_extraction(event))))),
            map(events => events.filter(([id, event]) => get_unit_id(event.subject) === get_unit_id(target_extraction(event)) || filter_target.has(get_unit_id(target_extraction(event))))),
            map(events => events.filter(([id, event]) => ability_extraction(event).every(ability => this.ability_filter$.has(ability)))),
            map(events => new Map(events))
        );
    }

    private apply_interval_and_source_filter_to_events(subject: Observable<Array<Event>>, source_extraction: (Event) => Unit, inverse_filter: boolean = false): Observable<Array<Event>> {
        const filter = inverse_filter ? this.target_filter$ : this.source_filter$;
        return subject.pipe(
            map(events => events.filter(event => this.attempt_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) !== undefined)),
            map(events => events.filter(event => filter.has(get_unit_id(source_extraction(event)))))
        );
    }

    private apply_target_filter_to_events(subject: Observable<Array<Event>>, target_extraction: (Event) => Unit, inverse_filter: boolean = false): Observable<Array<Event>> {
        const filter = inverse_filter ? this.source_filter$ : this.target_filter$;
        return subject.pipe(map(events => events.filter(event => get_unit_id(event.subject) === get_unit_id(target_extraction(event)) ||
            filter.has(get_unit_id(target_extraction(event))))));
    }

    private apply_ability_filter_to_events(subject: Observable<Array<Event>>, ability_extraction: (Event) => Array<number>): Observable<Array<Event>> {
        return subject.pipe(map(events => events.filter(event => ability_extraction(event).every(ability => this.ability_filter$.has(ability)))));
    }

    public get_spell_casts(inverse_filter: boolean = false): Observable<Map<number, Event>> {
        if (!this.spell_casts$) {
            this.spell_casts$ = new BehaviorSubject(new Map());
            this.load_instance_data(0, events => {
                this.subscriptions.push(this.spell_casts$.subscribe(i_events =>
                    this.extract_subjects([...i_events.values()], se_identity, te_spell_cast, ae_spell_cast)));
                this.spell_casts$.next(events.reduce((result, event) => {
                    result.set(event.id, event);
                    return result;
                }, new Map()));
                this.changed$.next(ChangedSubject.SpellCast);

                // To solve dependency issues
                this.interrupts$?.next(this.interrupts$.getValue());
                this.spell_steals$?.next(this.spell_steals$.getValue());
                this.dispels$?.next(this.dispels$.getValue());
                this.spell_damage$?.next(this.spell_damage$.getValue());
                this.heal$?.next(this.heal$.getValue());
                this.threat$?.next(this.threat$.getValue());

                this.register_load_instance(0, this.spell_casts$);
            }, 0);
        }
        return this.apply_filter_to_events_map(this.spell_casts$.asObservable(), se_identity, te_spell_cast, ae_spell_cast, inverse_filter);
    }

    public get_deaths(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.deaths$) {
            this.deaths$ = new BehaviorSubject([]);
            this.load_instance_data(1, events => {
                this.subscriptions.push(this.deaths$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.deaths$.next(events);
                this.changed$.next(ChangedSubject.Death);
                this.register_load_instance(1, this.deaths$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.deaths$.asObservable(), se_identity, inverse_filter);
    }

    public get_combat_states(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.combat_states$) {
            this.combat_states$ = new BehaviorSubject([]);
            this.load_instance_data(2, events => {
                this.subscriptions.push(this.combat_states$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.combat_states$.next(events);
                this.changed$.next(ChangedSubject.CombatState);
                this.register_load_instance(2, this.combat_states$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.combat_states$.asObservable(), se_identity, inverse_filter);
    }

    public get_loot(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.loot$) {
            this.loot$ = new BehaviorSubject([]);
            this.load_instance_data(3, events => {
                this.subscriptions.push(this.loot$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.loot$.next(events);
                this.changed$.next(ChangedSubject.Loot);
                this.register_load_instance(3, this.loot$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.loot$.asObservable(), se_identity, inverse_filter);
    }

    public get_positions(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.positions$) {
            this.positions$ = new BehaviorSubject([]);
            this.load_instance_data(4, events => {
                this.subscriptions.push(this.positions$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.positions$.next(events);
                this.changed$.next(ChangedSubject.Position);
                this.register_load_instance(4, this.positions$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.positions$.asObservable(), se_identity, inverse_filter);
    }

    public get_powers(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.powers$) {
            this.powers$ = new BehaviorSubject([]);
            this.load_instance_data(5, events => {
                this.subscriptions.push(this.powers$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.powers$.next(events);
                this.changed$.next(ChangedSubject.Power);
                this.register_load_instance(5, this.powers$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.powers$.asObservable(), se_identity, inverse_filter);
    }

    public get_aura_applications(inverse_filter: boolean = false): Observable<Map<number, Event>> {
        if (!this.aura_applications$) {
            this.aura_applications$ = new BehaviorSubject(new Map());
            this.load_instance_data(6, events => {
                this.subscriptions.push(this.aura_applications$.subscribe(i_events =>
                    this.extract_subjects([...i_events.values()], se_identity, te_aura_application, ae_aura_application)));
                this.aura_applications$.next(events.reduce((result, event) => {
                    result.set(event.id, event);
                    return result;
                }, new Map()));
                this.changed$.next(ChangedSubject.AuraApplication);

                // To solve dependency issues
                this.interrupts$?.next(this.interrupts$.getValue());
                this.spell_steals$?.next(this.spell_steals$.getValue());
                this.dispels$?.next(this.dispels$.getValue());

                this.register_load_instance(6, this.aura_applications$);
            }, 0);
        }
        return this.apply_filter_to_events_map(this.aura_applications$.asObservable(), se_identity, te_aura_application, ae_aura_application, inverse_filter);
    }

    public get_interrupts(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_spell_casts().pipe(take(1));
        this.get_aura_applications().pipe(take(1));
        if (!this.interrupts$) {
            this.interrupts$ = new BehaviorSubject([]);
            this.load_instance_data(7, events => {
                this.subscriptions.push(this.interrupts$.subscribe(i_events => this.extract_subjects(i_events, se_identity,
                    te_spell_cast_or_aura_app(ce_interrupt, this.spell_casts$.getValue(), this.aura_applications$.getValue()),
                    ae_interrupt(this.spell_casts$.getValue(), this.aura_applications$.getValue()))));
                this.interrupts$.next(events);
                this.changed$.next(ChangedSubject.Interrupt);
                this.register_load_instance(7, this.interrupts$);
            }, 0);
        }
        return this.apply_filter_to_events(this.interrupts$.asObservable(), se_identity,
            te_spell_cast_or_aura_app(ce_interrupt, this.spell_casts$.getValue(), this.aura_applications$.getValue()),
            ae_interrupt(this.spell_casts$.getValue(), this.aura_applications$.getValue()), inverse_filter);
    }

    public get_spell_steals(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_spell_casts().pipe(take(1));
        this.get_aura_applications().pipe(take(1));
        if (!this.spell_steals$) {
            this.spell_steals$ = new BehaviorSubject([]);
            this.load_instance_data(8, events => {
                this.subscriptions.push(this.spell_steals$.subscribe(i_events => this.extract_subjects(i_events, se_identity,
                    te_spell_cast_by_cause(ce_spell_steal, this.spell_casts$.getValue()), ae_spell_steal(this.spell_casts$.getValue(), this.aura_applications$.getValue()))));
                this.spell_steals$.next(events);
                this.changed$.next(ChangedSubject.SpellSteal);
                this.register_load_instance(8, this.spell_steals$);
            }, 0);
        }
        return this.apply_filter_to_events(this.spell_steals$.asObservable(), se_identity,
            te_spell_cast_by_cause(ce_spell_steal, this.spell_casts$.getValue()),
            ae_spell_steal(this.spell_casts$.getValue(), this.aura_applications$.getValue()), inverse_filter);
    }

    public get_dispels(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_spell_casts().pipe(take(1));
        this.get_aura_applications().pipe(take(1));
        if (!this.dispels$) {
            this.dispels$ = new BehaviorSubject([]);
            this.load_instance_data(9, events => {
                this.subscriptions.push(this.dispels$.subscribe(i_events => this.extract_subjects(i_events, se_identity,
                    te_spell_cast_by_cause(ce_dispel, this.spell_casts$.getValue()), ae_dispel(this.spell_casts$.getValue(), this.aura_applications$.getValue()))));
                this.dispels$.next(events);
                this.changed$.next(ChangedSubject.Dispel);
                this.register_load_instance(9, this.dispels$);
            }, 0);
        }
        return this.apply_filter_to_events(this.dispels$.asObservable(), se_identity,
            te_spell_cast_by_cause(ce_dispel, this.spell_casts$.getValue()),
            ae_dispel(this.spell_casts$.getValue(), this.aura_applications$.getValue()), inverse_filter);
    }

    public get_threat_wipes(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.threat_wipes$) {
            this.threat_wipes$ = new BehaviorSubject([]);
            this.load_instance_data(10, events => {
                this.subscriptions.push(this.threat_wipes$.subscribe(i_events => this.extract_subjects(i_events, se_identity, undefined, undefined)));
                this.threat_wipes$.next(events);
                this.changed$.next(ChangedSubject.ThreatWipe);
                this.register_load_instance(10, this.threat_wipes$);
            }, 0);
        }
        return this.apply_interval_and_source_filter_to_events(this.threat_wipes$.asObservable(), se_identity, inverse_filter);
    }

    public get_summons(inverse_filter: boolean = false): Observable<Array<Event>> {
        if (!this.summons$) {
            this.summons$ = new BehaviorSubject([]);
            this.load_instance_data(11, events => {
                this.subscriptions.push(this.summons$.subscribe(i_events =>
                    this.extract_subjects(i_events, se_identity, te_summon, undefined)));
                this.summons$.next(events);
                this.changed$.next(ChangedSubject.Summon);
                this.register_load_instance(11, this.summons$);
            }, 0);
        }
        return this.apply_target_filter_to_events(this.apply_interval_and_source_filter_to_events(this.summons$, se_identity), te_summon, inverse_filter);
    }

    public get_melee_damage(inverse_filter: boolean = false): Observable<Map<number, Event>> {
        if (!this.melee_damage$) {
            this.melee_damage$ = new BehaviorSubject(new Map());
            this.load_instance_data(12, events => {
                this.subscriptions.push(this.melee_damage$.subscribe(i_events =>
                    this.extract_subjects([...i_events.values()], se_identity, te_melee_damage, ae_melee_damage)));
                this.melee_damage$.next(events.reduce((result, event) => {
                    result.set(event.id, event);
                    return result;
                }, new Map()));
                this.changed$.next(ChangedSubject.MeleeDamage);

                // To solve dependency issues
                this.threat$?.next(this.threat$.getValue());

                this.register_load_instance(12, this.melee_damage$);
            }, 0);
        }
        return this.apply_filter_to_events_map(this.melee_damage$.asObservable(), se_identity, te_melee_damage, ae_melee_damage, inverse_filter);
    }

    public get_spell_damage(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_aura_applications().pipe(take(1));
        this.get_spell_casts().pipe(take(1));
        if (!this.spell_damage$) {
            this.spell_damage$ = new BehaviorSubject([]);
            this.load_instance_data(13, events => {
                this.subscriptions.push(this.spell_damage$.subscribe(i_events =>
                    this.extract_subjects(i_events, se_aura_app_or_own(ce_spell_damage, this.aura_applications$.getValue()),
                        te_spell_damage, ae_spell_cast_or_aura_application(ce_spell_damage, this.spell_casts$.getValue(), this.aura_applications$.getValue()))));
                this.spell_damage$.next(events);
                this.changed$.next(ChangedSubject.SpellDamage);
                this.register_load_instance(13, this.spell_damage$);
            }, 0);
        }
        return this.apply_filter_to_events(this.spell_damage$.asObservable(), se_aura_app_or_own(ce_spell_damage, this.aura_applications$.getValue()),
            te_spell_damage, ae_spell_cast_or_aura_application(ce_spell_damage, this.spell_casts$.getValue(), this.aura_applications$.getValue()), inverse_filter);
    }

    public get_heal(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_aura_applications().pipe(take(1));
        this.get_spell_casts().pipe(take(1));
        if (!this.heal$) {
            this.heal$ = new BehaviorSubject([]);
            this.load_instance_data(14, events => {
                this.subscriptions.push(this.heal$.subscribe(i_events =>
                    this.extract_subjects(i_events, se_aura_app_or_own(ce_heal, this.aura_applications$.getValue()),
                        te_heal, ae_spell_cast_or_aura_application(ce_heal, this.spell_casts$.getValue(), this.aura_applications$.getValue()))));
                this.heal$.next(events);
                this.changed$.next(ChangedSubject.Heal);
                this.register_load_instance(14, this.heal$);
            }, 0);
        }
        return this.apply_filter_to_events(this.heal$.asObservable(), se_aura_app_or_own(ce_heal, this.aura_applications$.getValue()),
            te_heal, ae_spell_cast_or_aura_application(ce_heal, this.spell_casts$.getValue(), this.aura_applications$.getValue()), inverse_filter);
    }

    public get_threat(inverse_filter: boolean = false): Observable<Array<Event>> {
        this.get_spell_casts().pipe(take(1));
        this.get_melee_damage().pipe(take(1));
        if (!this.threat$) {
            this.threat$ = new BehaviorSubject([]);
            this.load_instance_data(15, events => {
                this.subscriptions.push(this.threat$.subscribe(i_events =>
                    this.extract_subjects(i_events, se_identity, te_threat, ae_threat(this.spell_casts$.getValue()))));
                this.threat$.next(events);
                this.changed$.next(ChangedSubject.Threat);
                this.register_load_instance(15, this.threat$);
            }, 0);
        }
        return this.apply_filter_to_events(this.threat$.asObservable(), se_identity, te_threat, ae_threat(this.spell_casts$.getValue()), inverse_filter);
    }

    public get meta(): Observable<InstanceViewerMeta> {
        if (!this.instance_meta$) {
            this.instance_meta$ = new BehaviorSubject(undefined);
            this.load_instance_meta(meta => {
                this.instance_meta$.next(meta);
                this.changed$.next(ChangedSubject.InstanceMeta);
            });
            this.subscriptions.push(this.instance_meta$.subscribe(meta => {
                if (!!meta && !!meta.expired)
                    clearInterval(this.updater);
            }));
        }
        return this.instance_meta$.asObservable();
    }

    public get participants(): Observable<Array<InstanceViewerParticipants>> {
        if (!this.participants$) {
            this.participants$ = new BehaviorSubject([]);
            this.load_participants(participants => {
                this.participants$.next(participants);
                this.changed$.next(ChangedSubject.Participants);
            });
        }
        return this.participants$.asObservable();
    }

    public get attempts(): Observable<Array<InstanceViewerAttempt>> {
        if (!this.attempts$) {
            this.attempts$ = new BehaviorSubject([]);
            this.load_attempts(attempts => {
                this.attempts$.next(attempts);
                this.changed$.next(ChangedSubject.Attempts);
            });
        }
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
