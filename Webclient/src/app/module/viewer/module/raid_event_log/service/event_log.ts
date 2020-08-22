import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, combineLatest, forkJoin, Observable, of, Subject, Subscription} from "rxjs";
import {EventLogEntry} from "../domain_value/event_log_entry";
import {InstanceDataService} from "../../../service/instance_data";
import {debounceTime, map} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {UnitService} from "../../../service/unit";
import {
    get_aura_application,
    get_death,
    get_heal,
    get_melee_damage,
    get_spell_cast,
    get_spell_damage
} from "../../../extractor/events";
import {HitType} from "../../../domain_value/hit_type";
import {SpellService} from "../../../service/spell";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class EventLogService implements OnDestroy {

    private subscription: Subscription = new Subscription();
    private event_log_entries$: BehaviorSubject<Array<EventLogEntry>> = new BehaviorSubject([]);
    private to_actor: boolean = false;
    private initialized: boolean = false;

    private last_global_offset: number = 0;
    private log_offsets: Map<number, [number, Set<number>]> = new Map([
        [0, [0, new Set()]],
        [1, [0, new Set()]],
        [2, [0, new Set()]],
        [3, [0, new Set()]],
        [4, [0, new Set()]],
        [5, [0, new Set()]],
        [6, [0, new Set()]],
        [7, [0, new Set()]],
        [8, [0, new Set()]],
        [9, [0, new Set()]],
        [10, [0, new Set()]],
        [11, [0, new Set()]],
        [12, [0, new Set()]],
        [13, [0, new Set()]],
        [14, [0, new Set()]],
        [15, [0, new Set()]],
    ]);
    public offset_changed: Subject<number> = new Subject();

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService,
        private spellService: SpellService
    ) {
        this.subscription.add(this.offset_changed.asObservable().pipe(debounceTime(2)).subscribe(offset => {
            if (offset === 0) {
                // Reset
                for (const log_offset of this.log_offsets) {
                    log_offset[1][0] = 0;
                    log_offset[1][1].clear();
                }
            } else {
                if (this.last_global_offset > offset) {
                    // Scrolled up
                    for (let i = this.last_global_offset; i > offset; --i) {
                        for (const log_offset of this.log_offsets) {
                            if (log_offset[1][1].has(i)) {
                                log_offset[1][1].delete(i);
                                --log_offset[1][0];
                            }
                        }
                    }
                } else {
                    // Scrolled down
                    const log_entries = this.event_log_entries$.getValue();
                    for (let i = 0; i < (offset - this.last_global_offset); ++i) {
                        if (log_entries.length <= i)
                            break; // Scrolling too fast?
                        const log_entry = log_entries[i];
                        const log_offset = this.log_offsets.get(log_entry.type);
                        log_offset[1].add(offset + i);
                        ++log_offset[0];
                    }
                }
            }
            this.last_global_offset = offset;
            this.update_event_log_entries();
        }));
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get event_log_entries(): Observable<Array<EventLogEntry>> {
        if (!this.initialized) {
            this.update_event_log_entries();
            this.subscription.add(this.instanceDataService.knecht_updates.subscribe(knecht_update => {
                this.update_event_log_entries();
            }));
            this.initialized = true;
        }
        return this.event_log_entries$.asObservable().pipe(map(entries => entries.slice(0, 19)));
    }

    set_actor(to_actor: boolean): void {
        if (this.to_actor !== to_actor) {
            this.to_actor = to_actor;
            this.update_event_log_entries();
        }
    }

    private create_event_log_entry(type: number, event: Event, processor: (Event) => Observable<string>): EventLogEntry {
        return {
            id: event.id,
            type,
            timestamp: event.timestamp,
            event_message: processor(event)
        };
    }

    async get_event_log_entries(up_to_timestamp: number): Promise<Array<EventLogEntry>> {
        return [
            ...(await this.instanceDataService.knecht_melee.event_log_melee_damage(this.to_actor, this.log_offsets.get(12)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(12, event, (evt) => this.process_melee_damage(evt))),
            ...(await this.instanceDataService.knecht_spell.event_log_spell_damage(this.to_actor, this.log_offsets.get(13)[0], up_to_timestamp))
                .map(([event, [indicator, spell_cause_event]]) =>
                    this.create_event_log_entry(13, event, (evt) => this.process_spell_damage(evt, indicator, spell_cause_event))),
            ...(await this.instanceDataService.knecht_spell.event_log_heal(this.to_actor, this.log_offsets.get(14)[0], up_to_timestamp))
                .map(([event, [indicator, spell_cause_event]]) =>
                    this.create_event_log_entry(14, event, (evt) => this.process_heal(evt, indicator, spell_cause_event))),
            ...(await this.instanceDataService.knecht_melee.event_log_deaths(this.to_actor, this.log_offsets.get(1)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(1, event, (evt) => this.process_death(evt))),
        ].sort((left, right) => {
            const cmp = right.timestamp - left.timestamp;
            if (cmp === 0)
                return right.id - left.id;
            return cmp;
        });
    }

    private async update_event_log_entries(): Promise<void> {
        this.event_log_entries$.next(await this.get_event_log_entries(0));
    }

    // TODO: Mitigations!
    private process_melee_damage(event: Event): Observable<string> {
        const melee_damage_event = get_melee_damage(event);
        const subject = this.unitService.get_unit_name(event.subject);
        const victim = this.unitService.get_unit_name(melee_damage_event.victim);
        return combineLatest([subject, victim])
            .pipe(map((([subject_name, victim_name]) => {
                let hit_type_str = "hits";
                switch (melee_damage_event.hit_mask[0]) {
                    case HitType.Crit:
                        hit_type_str = "crits";
                        break;
                    case HitType.Dodge:
                        hit_type_str = "is dodged by";
                        break;
                    case HitType.Parry:
                        hit_type_str = "is parried by";
                        break;
                    case HitType.FullBlock:
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
    private process_spell_damage(event: Event, indicator: boolean, spell_cause_event: Event): Observable<string> {
        const spell_damage_event = get_spell_damage(event);
        if (!spell_cause_event)
            return of(CONST_UNKNOWN_LABEL);

        const spell_id = indicator ? get_spell_cast(spell_cause_event).spell_id : get_aura_application(spell_cause_event).spell_id;
        const subject$ = this.unitService.get_unit_name(event.subject);
        const victim$ = this.unitService.get_unit_name(spell_damage_event.damage.victim);
        const ability$ = this.spellService.get_spell_name(spell_id);
        return combineLatest([subject$, victim$, ability$])
            .pipe(map((([subject_name, victim_name, ability_name]) => {
                let hit_type_str = "hits";
                switch (spell_damage_event.damage.hit_mask[0]) {
                    case HitType.Crit:
                        hit_type_str = "crits";
                        break;
                    case HitType.FullResist:
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
    private process_heal(event: Event, indicator: boolean, spell_cause_event: Event): Observable<string> {
        const heal_event = get_heal(event);
        if (!spell_cause_event)
            return of(CONST_UNKNOWN_LABEL);
        const cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);

        const hit_type = !!(cause as any).hit_mask ? (cause as any).hit_mask[0] : HitType.Hit;
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
                    case HitType.FullAbsorb:
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
