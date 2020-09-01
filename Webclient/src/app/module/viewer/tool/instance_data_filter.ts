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

    constructor(instance_meta_id: number, event_types: Array<number>) {
        this.data_loader = new InstanceDataLoader(instance_meta_id, event_types);
        this.filter_changed$.asObservable().pipe(auditTime(250)).subscribe(() => {
            (self as any).postMessage(["KNECHT_UPDATES", KnechtUpdates.FilterChanged]);
        });
    }

    private apply_filter(container: Array<Event>, source_extraction: (Event) => Unit, target_extraction: (Event) => Unit,
                         multi_ability_extraction: (Event) => Array<number>, single_ability_extraction: (Event) => number,
                         inverse_filter: boolean = false): Array<Event> {
        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        const filter_target = inverse_filter ? this.source_filter$ : this.target_filter$;
        let result = container.filter(event => this.segment_intervals$.find(interval => {
            return interval[0] <= event[1] && interval[1] >= event[1];
        }) !== undefined).filter(event => filter_source.has(get_unit_id(source_extraction(event))));
        if (!!target_extraction)
            result = result.filter(event => filter_target.has(get_unit_id(target_extraction(event))));
        if (!!multi_ability_extraction)
            result = result.filter(event => multi_ability_extraction(event).every(ability => this.ability_filter$.has(ability)));
        if (!!single_ability_extraction)
            result = result.filter(event => this.ability_filter$.has(single_ability_extraction(event)));
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
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.spell_casts, se_spell_cast, te_spell_cast, undefined, ae_spell_cast, inverse_filter);
    }

    get_deaths(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.deaths, se_death, te_death, undefined, undefined, inverse_filter);
    }

    get_combat_states(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.combat_states, se_combat_state, undefined, undefined, undefined, inverse_filter);
    }

    get_loot(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.loot, se_loot, undefined, undefined, undefined, inverse_filter);
    }

    get_positions(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.positions, se_position, undefined, undefined, undefined, inverse_filter);
    }

    get_powers(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.powers, se_power, undefined, undefined, undefined, inverse_filter);
    }

    get_aura_applications(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.aura_applications, se_aura_application, te_aura_application, undefined, ae_aura_application, inverse_filter);
    }

    get_interrupts(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.interrupts, se_interrupt, te_interrupt, ae_interrupt, undefined, inverse_filter);
    }

    get_spell_steals(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.spell_steals, se_un_aura, te_un_aura, ae_un_aura, undefined, inverse_filter);
    }

    get_dispels(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.dispels, se_un_aura, te_un_aura, ae_un_aura, undefined, inverse_filter);
    }

    get_threat_wipes(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.threat_wipes, se_threat_wipe, undefined, undefined, undefined, inverse_filter);
    }

    get_summons(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.summons, se_summon, te_summon, undefined, undefined, inverse_filter);
    }

    get_melee_damage(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.melee_damage, se_melee_damage, te_melee_damage, undefined, ae_melee_damage, inverse_filter);
    }

    get_spell_damage(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.spell_damage, se_spell_damage, te_spell_damage, undefined, ae_spell_damage, inverse_filter);
    }

    get_heal(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.heal, se_heal, te_heal, undefined, ae_heal, inverse_filter);
    }

    get_threat(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        return this.apply_filter(this.data_loader.threat, se_threat, te_threat, undefined, ae_threat, inverse_filter);
    }

    get_non_segmented_aura_applications(inverse_filter: boolean = false): Array<Event> {
        if (!this.data_loader.initialized) return [];
        const filter_source = inverse_filter ? this.target_filter$ : this.source_filter$;
        return this.data_loader.aura_applications.filter(event => filter_source.has(get_unit_id(se_aura_application(event))))
            .filter(event => this.ability_filter$.has(ae_aura_application(event)));
    }

    get_segment_intervals(): Array<[number, number]> {
        return this.segment_intervals$;
    }
}
