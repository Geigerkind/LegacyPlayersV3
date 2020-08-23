import {InstanceDataLoader} from "./instance_data_loader";
import {Event} from "../domain_value/event";
import {get_unit_id, Unit} from "../domain_value/unit";
import {se_aura_app_or_own, se_dispel, se_identity, se_interrupt} from "../extractor/sources";
import {
    te_aura_application,
    te_death, te_dispel, te_heal, te_melee_damage,
    te_spell_cast,
    te_spell_cast_by_cause,
    te_spell_cast_or_aura_app, te_spell_damage, te_summon, te_threat
} from "../extractor/targets";
import {
    ae_aura_application,
    ae_dispel,
    ae_interrupt,
    ae_melee_damage,
    ae_spell_cast, ae_spell_cast_or_aura_application,
    ae_spell_steal, ae_threat
} from "../extractor/abilities";
import {ce_heal, ce_interrupt, ce_spell_damage, ce_spell_steal} from "../extractor/causes";
import {Observable, Subject} from "rxjs";
import {iterable_filterMap, iterable_some} from "../../../stdlib/iterable_higher_order";
import {debounceTime} from "rxjs/operators";
import {KnechtUpdates} from "../domain_value/knecht_updates";

export class InstanceDataFilter {

    private filter_changed$: Subject<void> = new Subject();

    private segment_intervals$: Array<[number, number]> = [];
    private source_filter$: Set<number> = new Set();
    private target_filter$: Set<number> = new Set();
    private ability_filter$: Set<number> = new Set();

    private data_loader: InstanceDataLoader;

    constructor(instance_meta_id: number, event_types: Array<number>) {
        this.data_loader = new InstanceDataLoader(instance_meta_id, event_types);
        this.filter_changed$.asObservable().pipe(debounceTime(500)).subscribe(() => {
            (self as any).postMessage(["KNECHT_UPDATES", KnechtUpdates.FilterChanged]);
        });
    }

    private apply_filter(container: Array<Event>, source_extraction: (Event) => Unit, target_extraction: (Event) => Unit,
                         ability_extraction: (Event) => Array<number>, inverse_filter: boolean = false): Array<Event> {
        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        const filter_target = inverse_filter ? this.source_filter$ : this.target_filter$;
        let result = container.filter(event => this.segment_intervals$.find(interval => interval[0] <= event.timestamp && interval[1] >= event.timestamp) !== undefined)
            .filter(event => filter_source.has(get_unit_id(source_extraction(event))));
        if (!!target_extraction)
            result = result.filter(event => filter_target.has(get_unit_id(target_extraction(event))));
        if (!!ability_extraction)
            result = result.filter(event => ability_extraction(event).every(ability => this.ability_filter$.has(ability)));
        return result;
    }

    async set_segment_intervals(intervals: Array<[number, number]>): Promise<void> {
        this.segment_intervals$ = intervals;
        this.filter_changed$.next();
    }

    async set_source_filter(sources: Array<number>): Promise<void> {
        this.source_filter$ = new Set(sources);
        this.filter_changed$.next();
    }

    async set_target_filter(targets: Array<number>): Promise<void> {
        this.target_filter$ = new Set(targets);
        this.filter_changed$.next();
    }

    async set_ability_filter(abilities: Array<number>): Promise<void> {
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
        return this.apply_filter(this.data_loader.spell_casts, se_identity, te_spell_cast, ae_spell_cast, inverse_filter);
    }

    get_deaths(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.deaths, se_identity, te_death, undefined, inverse_filter);
    }

    get_combat_states(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.combat_states, se_identity, undefined, undefined, inverse_filter);
    }

    get_loot(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.loot, se_identity, undefined, undefined, inverse_filter);
    }

    get_positions(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.positions, se_identity, undefined, undefined, inverse_filter);
    }

    get_powers(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.powers, se_identity, undefined, undefined, inverse_filter);
    }

    get_aura_applications(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.aura_applications, se_identity, te_aura_application, ae_aura_application, inverse_filter);
    }

    get_interrupts(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.interrupts, se_interrupt(this.get_event_map()), se_identity,
            ae_interrupt(this.data_loader.event_map), inverse_filter);
    }

    get_spell_steals(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.spell_steals, se_identity, te_spell_cast_by_cause(ce_spell_steal, this.data_loader.event_map),
            ae_spell_steal(this.data_loader.event_map), inverse_filter);
    }

    get_dispels(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.dispels, se_dispel(this.data_loader.event_map),
            te_dispel(this.data_loader.event_map), ae_dispel(this.data_loader.event_map), inverse_filter);
    }

    get_threat_wipes(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.threat_wipes, se_identity, undefined, undefined, inverse_filter);
    }

    get_summons(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.summons, se_identity, te_summon, undefined, inverse_filter);
    }

    get_melee_damage(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.melee_damage, se_identity, te_melee_damage, ae_melee_damage, inverse_filter);
    }

    get_spell_damage(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.spell_damage, se_aura_app_or_own(ce_spell_damage, this.data_loader.event_map),
            te_spell_damage, ae_spell_cast_or_aura_application(ce_spell_damage, this.data_loader.event_map), inverse_filter);
    }

    get_heal(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.heal, se_aura_app_or_own(ce_heal, this.data_loader.event_map),
            te_heal, ae_spell_cast_or_aura_application(ce_heal, this.data_loader.event_map), inverse_filter);
    }

    get_threat(inverse_filter: boolean = false): Array<Event> {
        return this.apply_filter(this.data_loader.threat, se_identity,
            te_threat, ae_threat(this.data_loader.event_map), inverse_filter);
    }

    get_event_map(): Map<number, Event> {
        return this.data_loader.event_map;
    }
}
