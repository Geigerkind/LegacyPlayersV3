import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, combineLatest, forkJoin, Observable, of, Subscription} from "rxjs";
import {EventLogEntry} from "../domain_value/event_log_entry";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {map, take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {UnitService} from "../../../service/unit";
import {
    get_aura_application,
    get_death,
    get_heal,
    get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_spell_damage
} from "../../../extractor/events";
import {HitType} from "../../../domain_value/hit_type";
import {SpellService} from "../../../service/spell";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class EventLogService implements OnDestroy {

    private subscription: Subscription;
    private event_log_entries$: BehaviorSubject<Array<EventLogEntry>> = new BehaviorSubject([]);

    private entry_melee_damage: Array<EventLogEntry> = [];
    private entry_spell_damage: Array<EventLogEntry> = [];
    private entry_heal: Array<EventLogEntry> = [];
    private entry_death: Array<EventLogEntry> = [];

    private to_actor: boolean = false;

    private initialized: boolean = false;

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService,
        private spellService: SpellService
    ) {
        this.subscription = this.instanceDataService.changed.subscribe(change => this.load_events(change));
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get event_log_entries(): Observable<Array<EventLogEntry>> {
        if (!this.initialized) {
            // this.initialized = true;
            this.load_events(ChangedSubject.Sources);
        }
        return this.event_log_entries$.asObservable();
    }

    set_actor(to_actor: boolean): void {
        if (this.to_actor !== to_actor) {
            this.to_actor = to_actor;
            this.load_events(ChangedSubject.Sources);
        }
    }

    private load_events(current_change: ChangedSubject): void {
        if (!this.initialized)
            return;

        this.load_on_change([ChangedSubject.MeleeDamage], current_change, () => {
            this.instanceDataService.get_melee_damage(this.to_actor).pipe(take(1))
                .subscribe(events => this.entry_melee_damage = [...events.values()]
                    .map(event => this.create_event_log_entry(event, (i_event) => this.process_melee_damage(i_event))));
        });
        this.load_on_change([ChangedSubject.SpellDamage, ChangedSubject.SpellCast, ChangedSubject.AuraApplication], current_change, () => {
            const spell_damage$ = this.instanceDataService.get_spell_damage(this.to_actor).pipe(take(1));
            const spell_casts$ = this.instanceDataService.get_spell_casts(this.to_actor).pipe(take(1));
            const aura_applications$ = this.instanceDataService.get_aura_applications(!this.to_actor).pipe(take(1));
            forkJoin([spell_damage$, spell_casts$, aura_applications$]).pipe(take(3))
                .subscribe(([spell_damage, spell_casts, aura_applications]) =>
                    this.entry_spell_damage = spell_damage.map(event => this.create_event_log_entry(event,
                        (i_event) => this.process_spell_damage(i_event, spell_casts, aura_applications))));
        });
        this.load_on_change([ChangedSubject.Heal, ChangedSubject.SpellCast, ChangedSubject.AuraApplication], current_change, () => {
            const heal$ = this.instanceDataService.get_heal(this.to_actor).pipe(take(1));
            const spell_casts$ = this.instanceDataService.get_spell_casts(this.to_actor).pipe(take(1));
            const aura_applications$ = this.instanceDataService.get_aura_applications(!this.to_actor).pipe(take(1));
            forkJoin([heal$, spell_casts$, aura_applications$]).pipe(take(3))
                .subscribe(([heal, spell_casts, aura_applications]) =>
                    this.entry_spell_damage = heal.map(event => this.create_event_log_entry(event,
                        (i_event) => this.process_heal(i_event, spell_casts, aura_applications))));
        });
        this.load_on_change([ChangedSubject.Death], current_change, () => {
            this.instanceDataService.get_deaths(this.to_actor).pipe(take(1))
                .subscribe(events => this.entry_death = events.map(event => this.create_event_log_entry(event, (i_event) => this.process_death(i_event))));
        });
    }

    private load_on_change(expected_change: Array<ChangedSubject>, current_change: ChangedSubject, update_function: () => void) {
        if (![ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Abilities, ChangedSubject.Attempts, ...expected_change].includes(current_change))
            return;
        update_function();
        this.update_event_log_entries();
    }

    private create_event_log_entry(event: Event, processor: (Event) => Observable<string>): EventLogEntry {
        return {
            id: event.id,
            timestamp: event.timestamp,
            event_message: processor(event)
        };
    }

    private update_event_log_entries(): void {
        this.event_log_entries$.next([
            ...this.entry_melee_damage,
            ...this.entry_spell_damage,
            ...this.entry_heal,
            ...this.entry_death,
        ].sort((left, right) => right.timestamp - left.timestamp));
    }

    // TODO: Mitigations!
    private process_melee_damage(event: Event): Observable<string> {
        const melee_damage_event = get_melee_damage(event);
        const subject = this.unitService.get_unit_name(event.subject);
        const victim = this.unitService.get_unit_name(melee_damage_event.victim);
        return combineLatest([subject, victim])
            .pipe(map((([subject_name, victim_name]) => {
                let hit_type_str = "hits";
                switch (melee_damage_event.hit_type) {
                    case HitType.Crit:
                        hit_type_str = "crits";
                        break;
                    case HitType.Dodge:
                        hit_type_str = "is dodged by";
                        break;
                    case HitType.Parry:
                        hit_type_str = "is parried by";
                        break;
                    case HitType.Block:
                        hit_type_str = "is blocked by";
                        break;
                    case HitType.Crushing:
                        hit_type_str = "crushes";
                        break;
                    case HitType.Glancing:
                        hit_type_str = "glances";
                        break;
                    case HitType.Evade:
                        hit_type_str = "is evaded by";
                        break;
                    case HitType.Miss:
                        hit_type_str = "is missed by";
                        break;
                    case HitType.Immune:
                        return subject_name + " attempts to hit, but " + victim_name + " is immune.";
                }
                if (melee_damage_event.damage === 0)
                    return subject_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + " " + hit_type_str + " " + victim_name + " for " + melee_damage_event.damage + ".";
            })));
    }

    // TODO: Mitigations!
    private process_spell_damage(event: Event, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): Observable<string> {
        const spell_damage_event = get_spell_damage(event);
        const [indicator, cause_event] = get_spell_cause(spell_damage_event.spell_cause_id, spell_casts, aura_applications);
        if (!cause_event)
            return of(CONST_UNKNOWN_LABEL);

        const spell_id = indicator ? get_spell_cast(cause_event).spell_id : get_aura_application(cause_event).spell_id;
        const subject$ = this.unitService.get_unit_name(event.subject);
        const victim$ = this.unitService.get_unit_name(spell_damage_event.damage.victim);
        const ability$ = this.spellService.get_spell_name(spell_id);
        return combineLatest([subject$, victim$, ability$])
            .pipe(map((([subject_name, victim_name, ability_name]) => {
                let hit_type_str = "hits";
                switch (spell_damage_event.damage.hit_type) {
                    case HitType.Crit:
                        hit_type_str = "crits";
                        break;
                    case HitType.Resist:
                    case HitType.Miss:
                        hit_type_str = "is resisted by";
                        break;
                    case HitType.Evade:
                        hit_type_str = "is evaded by";
                        break;
                    case HitType.Immune:
                        return subject_name + "'s " + ability_name + " attempts to hit, but " + victim_name + " is immune.";
                }
                if (spell_damage_event.damage.damage === 0)
                    return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + " for " + spell_damage_event.damage.damage + ".";
            })));
    }

    // TODO: Mitigations!
    private process_heal(event: Event, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>): Observable<string> {
        const heal_event = get_heal(event);
        const [indicator, cause_event] = get_spell_cause(heal_event.spell_cause_id, spell_casts, aura_applications);
        if (!cause_event)
            return of(CONST_UNKNOWN_LABEL);
        const cause = indicator ? get_spell_cast(cause_event) : get_aura_application(cause_event);

        const hit_type = !!(cause as any).hit_type ? (cause as any).hit_type : HitType.Hit;
        const spell_id = cause.spell_id;
        const subject$ = this.unitService.get_unit_name(event.subject);
        const victim$ = this.unitService.get_unit_name(heal_event.heal.target);
        const ability$ = this.spellService.get_spell_name(spell_id);
        return combineLatest([subject$, victim$, ability$])
            .pipe(map((([subject_name, victim_name, ability_name]) => {
                let hit_type_str = "heals";
                switch (hit_type) {
                    case HitType.Crit:
                        hit_type_str = "critically heals";
                        break;
                    case HitType.Absorb:
                        hit_type_str = "is absorbed by";
                        break;
                    case HitType.Evade:
                        hit_type_str = "is evaded by";
                        break;
                    case HitType.Immune:
                        return subject_name + "'s " + ability_name + " attempts to heal, but " + victim_name + " is immune.";
                }
                const overheal = heal_event.heal.total - heal_event.heal.effective;
                if (heal_event.heal.total === 0)
                    return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + " for " + heal_event.heal.effective + (overheal > 0 ? " (" + overheal + " overheal)." : ".");
            })));
    }

    private process_death(event: Event): Observable<string> {
        const death_event = get_death(event);
        const subject$ = this.unitService.get_unit_name(event.subject);
        const murder$ = this.unitService.get_unit_name(death_event.murder);
        return combineLatest([subject$, murder$])
            .pipe(map((([subject_name, murder_name]) => {
                return subject_name + " is killed by " + murder_name + ".";
            })));
    }
}
