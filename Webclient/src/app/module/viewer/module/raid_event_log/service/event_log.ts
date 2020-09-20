import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, combineLatest, Observable, of, Subject, Subscription} from "rxjs";
import {EventLogEntry} from "../domain_value/event_log_entry";
import {InstanceDataService} from "../../../service/instance_data";
import {debounceTime, map} from "rxjs/operators";
import {Event, MeleeDamage, SpellCast, SpellDamage} from "../../../domain_value/event";
import {UnitService} from "../../../service/unit";
import {hit_mask_to_hit_type_array, HitType} from "../../../domain_value/hit_type";
import {SpellService} from "../../../service/spell";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import {
    se_aura_application,
    se_combat_state, se_death, se_heal,
    se_interrupt, se_melee_damage,
    se_spell_cast, se_spell_damage,
    se_un_aura
} from "../../../extractor/sources";
import {
    te_aura_application, te_death, te_heal,
    te_interrupt,
    te_melee_damage,
    te_spell_cast, te_spell_damage,
    te_un_aura
} from "../../../extractor/targets";
import {
    ae_aura_application, ae_heal,
    ae_interrupt,
    ae_spell_cast,
    ae_spell_damage,
    ae_un_aura
} from "../../../extractor/abilities";
import {get_spell_components_total_amount, SpellComponent} from "../../../domain_value/damage";
import {school_mask_to_school_array} from "../../../domain_value/school";

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
            this.subscription.add(this.instanceDataService.knecht_updates.subscribe(([knecht_update, _]) => {
                if (knecht_update.some(elem => [KnechtUpdates.NewData, KnechtUpdates.FilterChanged].includes(elem)))
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
            id: event[0],
            type,
            timestamp: event[1],
            event_message: processor(event)
        };
    }

    async get_event_log_entries(up_to_timestamp: number): Promise<Array<EventLogEntry>> {
        return [
            ...(await this.instanceDataService.knecht_spell_cast.event_log_spell_cast(this.to_actor, this.log_offsets.get(0)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(0, event, (evt) => this.process_spell_cast(evt))),
            ...(await this.instanceDataService.knecht_melee.event_log_deaths(this.to_actor, this.log_offsets.get(1)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(1, event, (evt) => this.process_death(evt))),
            ...(await this.instanceDataService.knecht_replay.event_log_combat_state(this.to_actor, this.log_offsets.get(2)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(2, event, (evt) => this.process_combat_state(evt))),
            ...(await this.instanceDataService.knecht_aura.event_log_aura_application(this.to_actor, this.log_offsets.get(6)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(6, event, (evt) => this.process_aura_application(evt))),
            ...(await this.instanceDataService.knecht_interrupt.event_log_interrupt(this.to_actor, this.log_offsets.get(7)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(7, event, (evt) => this.process_interrupt(evt))),
            ...(await this.instanceDataService.knecht_spell_steal.event_log_spell_steal(this.to_actor, this.log_offsets.get(8)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(8, event, (evt) => this.process_spell_steal(evt))),
            ...(await this.instanceDataService.knecht_dispel.event_log_dispel(this.to_actor, this.log_offsets.get(9)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(9, event, (evt) => this.process_dispel(evt))),
            ...(await this.instanceDataService.knecht_melee.event_log_melee_damage(this.to_actor, this.log_offsets.get(12)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(12, event, (evt) => this.process_melee_damage(evt))),
            ...(await this.instanceDataService.knecht_spell_damage.event_log_spell_damage(this.to_actor, this.log_offsets.get(13)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(13, event, (evt) => this.process_spell_damage(evt))),
            ...(await this.instanceDataService.knecht_heal.event_log_heal(this.to_actor, this.log_offsets.get(14)[0], up_to_timestamp))
                .map(event => this.create_event_log_entry(14, event, (evt) => this.process_heal(evt))),
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

    private process_spell_cast(event: Event): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_spell_cast(event));
        const victim$ = this.unitService.get_unit_name(te_spell_cast(event));
        const spell$ = this.spellService.get_spell_name(ae_spell_cast(event));
        return combineLatest([subject$, victim$, spell$])
            .pipe(map((([subject_name, victim_name, spell_name]) => {
                return subject_name + " casts " + spell_name + " onto " + victim_name + ".";
            })));
    }

    private process_combat_state(event: Event): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_combat_state(event));
        return combineLatest([subject$])
            .pipe(map((([subject_name]) => {
                if (event[3])
                    return subject_name + " enters combat.";
                return subject_name + " leaves combat.";
            })));
    }

    private process_aura_application(event: Event): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_aura_application(event));
        const caster$ = this.unitService.get_unit_name(te_aura_application(event));
        const spell$ = this.spellService.get_spell_name(ae_aura_application(event));
        return combineLatest([subject$, caster$, spell$])
            .pipe(map((([subject_name, caster_name, spell_name]) => {
                if (event[5] === 0)
                    return spell_name + " (by " + caster_name + ") fades from " + subject_name + ".";
                return subject_name + " gains " + spell_name + " (by " + caster_name + ").";
            })));
    }

    private process_interrupt(event: Event): Observable<string> {
        const spells = ae_interrupt(event);
        const subject$ = this.unitService.get_unit_name(se_interrupt(event));
        const victim$ = this.unitService.get_unit_name(te_interrupt(event));
        const ability$ = this.spellService.get_spell_name(spells[0]);
        const interrupted_ability$ = this.spellService.get_spell_name(spells[1]);
        return combineLatest([subject$, victim$, ability$, interrupted_ability$])
            .pipe(map((([subject_name, victim_name, ability_name, interrupted_ability_name]) => {
                return subject_name + " interrupts " + victim_name + "'s " + interrupted_ability_name + " with " + ability_name + ".";
            })));
    }

    private process_spell_steal(event: Event): Observable<string> {
        const spells = ae_un_aura(event);
        const subject$ = this.unitService.get_unit_name(se_un_aura(event));
        const victim$ = this.unitService.get_unit_name(te_un_aura(event));
        const ability$ = this.spellService.get_spell_name(spells[0]);
        const stolen_ability$ = this.spellService.get_spell_name(spells[1]);
        return combineLatest([subject$, victim$, ability$, stolen_ability$])
            .pipe(map((([subject_name, victim_name, ability_name, stolen_ability_name]) => {
                return subject_name + " steals " + victim_name + "'s " + stolen_ability_name + " with " + ability_name + ".";
            })));
    }

    private process_dispel(event: Event): Observable<string> {
        const spells = ae_un_aura(event);
        const subject$ = this.unitService.get_unit_name(se_un_aura(event));
        const victim$ = this.unitService.get_unit_name(te_un_aura(event));
        const ability$ = this.spellService.get_spell_name(spells[0]);
        const dispelled_ability$ = this.spellService.get_spell_name(spells[1]);
        return combineLatest([subject$, victim$, ability$, dispelled_ability$])
            .pipe(map((([subject_name, victim_name, ability_name, dispelled_ability_name]) => {
                return subject_name + " dispels " + victim_name + "'s " + dispelled_ability_name + " with " + ability_name + ".";
            })));
    }

    private process_melee_damage(event: MeleeDamage): Observable<string> {
        const subject = this.unitService.get_unit_name(se_melee_damage(event));
        const victim = this.unitService.get_unit_name(te_melee_damage(event));
        return combineLatest([subject, victim])
            .pipe(map((([subject_name, victim_name]) => {
                const hit_type_str = this.get_hit_type_localization(hit_mask_to_hit_type_array(event[4]));
                const damage_done = get_spell_components_total_amount(event[5]);
                if (damage_done === 0)
                    return subject_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + " " + hit_type_str + " " + victim_name + " for " +
                    event[5].map(component => this.get_spell_component_localization(component)).join(", ") + ".";
            })));
    }

    private process_spell_damage(event: SpellDamage): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_spell_damage(event));
        const victim$ = this.unitService.get_unit_name(te_spell_damage(event));
        const ability$ = this.spellService.get_spell_name(ae_spell_damage(event));
        return combineLatest([subject$, victim$, ability$])
            .pipe(map((([subject_name, victim_name, ability_name]) => {
                const hit_type_str = this.get_hit_type_localization(hit_mask_to_hit_type_array(event[6]));
                const damage_done = get_spell_components_total_amount(event[7]);
                if (damage_done === 0)
                    return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + " for " +
                    event[7].map(component => this.get_spell_component_localization(component)).join(", ") + ".";
            })));
    }

    private process_heal(event: Event): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_heal(event));
        const victim$ = this.unitService.get_unit_name(te_heal(event));
        const ability$ = this.spellService.get_spell_name(ae_heal(event));
        return combineLatest([subject$, victim$, ability$])
            .pipe(map((([subject_name, victim_name, ability_name]) => {
                const hit_type_str = this.get_hit_type_localization(hit_mask_to_hit_type_array(event[6]), true);
                const overheal = event[8] - event[9];
                if (event[8] === 0)
                    return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + ".";
                return subject_name + "'s " + ability_name + " " + hit_type_str + " " + victim_name + " for " +
                    event[9] + this.get_mitigation_localization(event[10], 0, 0) +
                    (overheal > 0 ? " (" + overheal + " overheal)." : ".");
            })));
    }

    private process_death(event: Event): Observable<string> {
        const subject$ = this.unitService.get_unit_name(se_death(event));
        const murder$ = this.unitService.get_unit_name(te_death(event));
        return combineLatest([subject$, murder$])
            .pipe(map((([subject_name, murder_name]) => {
                if (murder_name === CONST_UNKNOWN_LABEL)
                    return subject_name + " died.";
                return subject_name + " is killed by " + murder_name + ".";
            })));
    }

    private get_hit_type_localization(hit_mask: Array<HitType>, is_heal: boolean = false): string {
        for (const hit_type of hit_mask) {
            switch (hit_type) {
                case HitType.Hit:
                    if (is_heal)
                        return "heals";
                    return "hits";
                case HitType.Crit:
                    if (is_heal)
                        return "critically heals";
                    return "crits";
                case HitType.Dodge:
                    return "is dodged by";
                case HitType.Parry:
                    return "is parried by";
                case HitType.FullBlock:
                    return "is blocked by";
                case HitType.FullResist:
                    return "is resisted by";
                case HitType.FullAbsorb:
                    return "is absorbed by";
                case HitType.Crushing:
                    return "crushes";
                case HitType.Glancing:
                    return "glances";
                case HitType.Evade:
                    return "is evaded by";
                case HitType.Miss:
                    return "is missed by";
                case HitType.Immune:
                    return "is ignored (immunity) by";
            }
        }
        return "?!?!";
    }

    private get_spell_component_localization(component: SpellComponent): string {
        return component[0] + " (" + school_mask_to_school_array(component[1]).map(school => school.toLowerCase()).join(", ") + ")" +
            this.get_mitigation_localization(component[2], component[3], component[4]);
    }

    private get_mitigation_localization(absorb: number, resist: number, block: number): string {
        const mitigation_str = [];
        if (absorb > 0)
            mitigation_str.push("(" + absorb + " absorbed)");
        if (resist > 0)
            mitigation_str.push("(" + resist + " resisted)");
        if (block > 0)
            mitigation_str.push("(" + block + " blocked)");
        return mitigation_str.length > 0 ? " " + mitigation_str.join(" ") : "";
    }
}
