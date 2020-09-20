import {InstanceDataLoader} from "./instance_data_loader";
import {Event} from "../domain_value/event";
import {get_unit_id, Unit} from "../domain_value/unit";
import {Observable, Subject} from "rxjs";
import {iterable_filterMap, iterable_some} from "../../../stdlib/iterable_higher_order";
import {auditTime} from "rxjs/operators";
import {KnechtUpdates} from "../domain_value/knecht_updates";
import {
    se_aura_application,
    se_combat_state,
    se_death, se_heal, se_interrupt,
    se_loot, se_melee_damage,
    se_position,
    se_power,
    se_spell_cast, se_spell_damage, se_summon, se_threat, se_threat_wipe, se_un_aura
} from "../extractor/sources";
import {
    te_aura_application,
    te_death, te_heal,
    te_interrupt,
    te_melee_damage,
    te_spell_cast, te_spell_damage,
    te_summon, te_threat,
    te_un_aura
} from "../extractor/targets";
import {
    ae_aura_application, ae_heal,
    ae_interrupt,
    ae_melee_damage,
    ae_spell_cast,
    ae_spell_damage, ae_threat,
    ae_un_aura
} from "../extractor/abilities";

export class InstanceDataFilter {

    private filter_changed$: Subject<void> = new Subject();

    private segment_intervals$: Array<[number, number]> = [];
    private source_filter$: Set<number> = new Set();
    private target_filter$: Set<number> = new Set();
    private ability_filter$: Set<number> = new Set();

    private data_loader: InstanceDataLoader;
    private cache: Map<string, Array<Event>> = new Map();

    constructor(instance_meta_id: number, event_types: Array<number>) {
        this.data_loader = new InstanceDataLoader(instance_meta_id, event_types);
        this.filter_changed$.asObservable().pipe(auditTime(250)).subscribe(() => {
            (self as any).postMessage(["KNECHT_UPDATES", KnechtUpdates.FilterChanged]);
        });
    }

    private apply_filter(event_type: number, container: Array<Event>, source_extraction: (Event) => number, target_extraction: (Event) => number,
                         multi_ability_extraction: (Event) => Array<number>, single_ability_extraction: (Event) => number,
                         inverse_filter: boolean = false): Array<Event> {
        const cache_key = event_type.toString() + (inverse_filter ? "1" : "0");
        if (this.cache.has(cache_key))
            return this.cache.get(cache_key);

        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        const filter_target = inverse_filter ? this.source_filter$ : this.target_filter$;
        let result = container.filter(event => this.segment_intervals$.find(interval => {
            return interval[0] <= event[1] && interval[1] >= event[1];
        }) !== undefined).filter(event => filter_source.has(source_extraction(event)));
        if (!!target_extraction)
            result = result.filter(event => {
                const unit_id = target_extraction(event);
                if (unit_id === 0)
                    return true;
                return filter_target.has(unit_id);
            });
        if (!!multi_ability_extraction)
            result = result.filter(event => multi_ability_extraction(event).every(ability => this.ability_filter$.has(ability)));
        if (!!single_ability_extraction)
            result = result.filter(event => this.ability_filter$.has(single_ability_extraction(event)));

        this.cache.set(cache_key, result);
        return result;
    }

    async set_segment_intervals(intervals: Array<[number, number]>): Promise<void> {
        this.cache = new Map();
        this.segment_intervals$ = intervals;
        this.filter_changed$.next();
    }

    async set_source_filter(sources: Array<number>): Promise<void> {
        this.cache = new Map();
        this.source_filter$ = new Set(sources);
        this.filter_changed$.next();
    }

    async set_target_filter(targets: Array<number>): Promise<void> {
        this.cache = new Map();
        this.target_filter$ = new Set(targets);
        this.filter_changed$.next();
    }

    async set_ability_filter(abilities: Array<number>): Promise<void> {
        this.cache = new Map();
        this.ability_filter$ = new Set(abilities);
        this.filter_changed$.next();
    }

    get filter_changed(): Observable<void> {
        return this.filter_changed$.asObservable();
    }

    async get_sources(): Promise<Array<Unit>> {
        return iterable_filterMap(this.data_loader.sources.values(), ([unit, occurrences]) =>
            iterable_some(occurrences.values(), timestamp => this.segment_intervals$
                .find(interval => interval[0] <= timestamp && interval[1] >= timestamp) !== undefined) ? unit : undefined);
    }

    async get_targets(): Promise<Array<Unit>> {
        return iterable_filterMap(this.data_loader.targets.values(), ([unit, occurrences]) =>
            iterable_some(occurrences.values(), timestamp => this.segment_intervals$
                .find(interval => interval[0] <= timestamp && interval[1] >= timestamp) !== undefined) ? unit : undefined);
    }

    async get_abilities(): Promise<Array<number>> {
        return iterable_filterMap(this.data_loader.abilities.entries(), ([ability_id, occurrences]) =>
            iterable_some(occurrences.values(), timestamp => this.segment_intervals$
                .find(interval => interval[0] <= timestamp && interval[1] >= timestamp) !== undefined) ? ability_id : undefined);
    }

    get_spell_casts(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(0, this.data_loader.spell_casts, (evt) => get_unit_id(se_spell_cast(evt)),
            (evt) => get_unit_id(te_spell_cast(evt), false), undefined, ae_spell_cast, inverse_filter);
    }

    get_deaths(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(1, this.data_loader.deaths, (evt) => get_unit_id(se_death(evt), false),
            (evt) => get_unit_id(te_death(evt), false), undefined, undefined, inverse_filter);
    }

    get_combat_states(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(2, this.data_loader.combat_states, (evt) => get_unit_id(se_combat_state(evt)),
            undefined, undefined, undefined, inverse_filter);
    }

    get_loot(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(3, this.data_loader.loot, (evt) => get_unit_id(se_loot(evt), false),
            undefined, undefined, undefined, inverse_filter);
    }

    get_positions(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(4, this.data_loader.positions, (evt) => get_unit_id(se_position(evt), false),
            undefined, undefined, undefined, inverse_filter);
    }

    get_powers(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(5, this.data_loader.powers, (evt) => get_unit_id(se_power(evt), false),
            undefined, undefined, undefined, inverse_filter);
    }

    get_aura_applications(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(6, this.data_loader.aura_applications, (evt) => get_unit_id(se_aura_application(evt), false),
            (evt) => get_unit_id(te_aura_application(evt), false), undefined, ae_aura_application, inverse_filter);
    }

    get_interrupts(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(7, this.data_loader.interrupts, (evt) => get_unit_id(se_interrupt(evt)),
            (evt) => get_unit_id(te_interrupt(evt), false), ae_interrupt, undefined, inverse_filter);
    }

    get_spell_steals(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(8, this.data_loader.spell_steals, (evt) => get_unit_id(se_un_aura(evt)),
            (evt) => get_unit_id(te_un_aura(evt), false), ae_un_aura, undefined, inverse_filter);
    }

    get_dispels(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(9, this.data_loader.dispels, (evt) => get_unit_id(se_un_aura(evt)),
            (evt) => get_unit_id(te_un_aura(evt), false), ae_un_aura, undefined, inverse_filter);
    }

    get_threat_wipes(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(10, this.data_loader.threat_wipes, (evt) => get_unit_id(se_threat_wipe(evt), false),
            undefined, undefined, undefined, inverse_filter);
    }

    get_summons(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(11, this.data_loader.summons, (evt) => get_unit_id(se_summon(evt), false),
            (evt) => get_unit_id(te_summon(evt), false), undefined, undefined, inverse_filter);
    }

    get_melee_damage(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(12, this.data_loader.melee_damage, (evt) => get_unit_id(se_melee_damage(evt)),
            (evt) => get_unit_id(te_melee_damage(evt), false), undefined, ae_melee_damage, inverse_filter);
    }

    get_spell_damage(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(13, this.data_loader.spell_damage, (evt) => get_unit_id(se_spell_damage(evt)),
            (evt) => get_unit_id(te_spell_damage(evt), false), undefined, ae_spell_damage, inverse_filter);
    }

    get_heal(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(14, this.data_loader.heal, (evt) => get_unit_id(se_heal(evt)),
            (evt) => get_unit_id(te_heal(evt), false), undefined, ae_heal, inverse_filter);
    }

    get_threat(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(15, this.data_loader.threat, (evt) => get_unit_id(se_threat(evt), false),
            (evt) => get_unit_id(te_threat(evt), false), undefined, ae_threat, inverse_filter);
    }

    get_non_segmented_aura_applications(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        return this.data_loader.aura_applications.filter(event => filter_source.has(get_unit_id(se_aura_application(event), false)))
            .filter(event => this.ability_filter$.has(ae_aura_application(event)));
    }

    get_segment_intervals(): Array<[number, number]> {
        return this.segment_intervals$;
    }
}
