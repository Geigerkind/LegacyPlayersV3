import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, combineLatest, Observable, Subject, Subscription, zip} from "rxjs";
import {Category} from "../domain_value/category";
import {InstanceViewerAttempt} from "../../../domain_value/instance_viewer_attempt";
import {Segment} from "../domain_value/segment";
import {EventSource} from "../domain_value/event_source";
import {RaidOption} from "../domain_value/raid_option";
import {debounceTime, map, take, takeUntil} from "rxjs/operators";
import {UnitService} from "../../../service/unit";
import {get_unit_id, is_player, Unit} from "../../../domain_value/unit";
import {InstanceViewerMeta} from "../../../domain_value/instance_viewer_meta";
import {EventAbility} from "../domain_value/event_ability";
import {SpellService} from "../../../service/spell";
import {DataService} from "../../../../../service/data";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {ViewerMode} from "../../../domain_value/viewer_mode";
import {ActivatedRoute, NavigationExtras, NavigationStart, Router} from "@angular/router";
import {iterable_map, iterable_some} from "../../../../../stdlib/iterable_higher_order";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";
import DataIntervalTree from "node-interval-tree";

export interface FilterStackItem {
    viewer_mode: ViewerMode;
    categories: Set<number>;
    segments: Set<number>;
    sources: Set<number>;
    targets: Set<number>;
    abilities: Set<number>;
}

@Injectable({
    providedIn: "root",
})
export class RaidConfigurationService implements OnDestroy {

    private subscription: Subscription = new Subscription();

    private current_meta: InstanceViewerMeta;

    private current_source_intervals: DataIntervalTree<number> = new DataIntervalTree();
    private current_target_intervals: DataIntervalTree<number> = new DataIntervalTree();
    private current_ability_intervals: DataIntervalTree<number> = new DataIntervalTree();

    private current_source_map: Map<number, Unit> = new Map();
    private current_target_map: Map<number, Unit> = new Map();
    private current_ability_map: Map<number, Set<number>> = new Map();

    private current_filtered_intervals: Array<[number, number]> = [];

    private categories$: BehaviorSubject<Array<Category>> = new BehaviorSubject([]);
    private segments$: BehaviorSubject<Array<Segment>> = new BehaviorSubject([]);
    private sources$: BehaviorSubject<Array<EventSource>> = new BehaviorSubject([]);
    private targets$: BehaviorSubject<Array<EventSource>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Array<EventAbility>> = new BehaviorSubject([]);
    private options$: BehaviorSubject<Array<RaidOption>> = new BehaviorSubject([]);

    private category_filter: Set<number> = new Set();
    private segment_filter: Set<number> = new Set();
    private source_filter: Set<number> = new Set();
    private target_filter: Set<number> = new Set();
    private ability_filter: Set<number> = new Set();

    private current_mode: ViewerMode = ViewerMode.Base;
    private prev_category_filter: Set<number> = new Set();
    private prev_segment_filter: Set<number> = new Set();
    private prev_source_filter: Set<number> = new Set();
    private prev_target_filter: Set<number> = new Set();
    private prev_ability_filter: Set<number> = new Set();

    private nextCategories: Subject<void> = new Subject();
    private nextSegments: Subject<void> = new Subject();
    private nextSources: Subject<void> = new Subject();
    private nextTargets: Subject<void> = new Subject();
    private nextAbilities: Subject<void> = new Subject();

    private filter_stack: Array<FilterStackItem> = [];
    private filter_updated$: Subject<boolean> = new Subject();
    private current_event_types: Set<number> = new Set();

    public selection_overwrite$: Subject<FilterStackItem> = new Subject();

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService,
        private spellService: SpellService,
        private dataService: DataService,
        private activated_route_service: ActivatedRoute,
        private router_service: Router
    ) {
        this.subscription.add(this.instanceDataService.meta.subscribe(meta => {
            this.current_meta = meta;
            this.instanceDataService.attempts.pipe(take(1)).subscribe(attempts => {
                this.update_categories(attempts);
                this.update_segments(attempts);
            });
        }));
        this.subscription.add(this.instanceDataService.attempts.subscribe(attempts => {
            this.update_categories(attempts);
            this.update_segments(attempts);
        }));

        this.subscription.add(this.instanceDataService.knecht_updates.subscribe(([knecht_updates, evt_types]) => {
            if (knecht_updates.some(elem => [KnechtUpdates.NewData, KnechtUpdates.Initialized].includes(elem))) {
                this.update_subjects();
            }
            if (knecht_updates.includes(KnechtUpdates.WorkerInitialized)) {
                this.instanceDataService.set_segment_intervals(this.current_filtered_intervals);
                this.instanceDataService.set_source_filter([...this.source_filter.values()]);
                this.instanceDataService.set_target_filter([...this.target_filter.values()]);
                this.instanceDataService.set_ability_filter([...this.ability_filter.values()]);
            }
        }));

        this.subscription.add(this.activated_route_service.paramMap.subscribe(params => this.current_mode = params.get("mode") as ViewerMode));
        this.subscription.add(this.filter_updated$.pipe(debounceTime(100)).subscribe(push_history => {
            if (push_history)
                this.router_service.navigate([this.router_service.url], { queryParams: { page: this.filter_stack.length } } as NavigationExtras);
            this.filter_stack.push({
                viewer_mode: this.current_mode,
                categories: new Set([...this.prev_category_filter.values()]),
                segments: new Set([...this.prev_segment_filter.values()]),
                sources: new Set(this.prev_source_filter),
                targets: new Set(this.prev_target_filter),
                abilities: new Set(this.prev_ability_filter)
            });
        }));

        this.subscription.add(this.router_service.events.subscribe((event: NavigationStart) => {
            if (event.navigationTrigger === "popstate") {
                const last_stack_entry = this.filter_stack.pop();
                if (last_stack_entry !== undefined) {
                    this.update_category_filter([...last_stack_entry.categories.values()]);
                    this.update_segment_filter([...last_stack_entry.segments.values()]);
                    this.update_source_filter([...last_stack_entry.sources.values()]);
                    this.update_target_filter([...last_stack_entry.targets.values()]);
                    this.update_ability_filter([...last_stack_entry.abilities.values()]);
                    this.selection_overwrite$.next(last_stack_entry);
                }
            }
        }));
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
        this.nextCategories.next();
        this.nextSegments.next();
        this.nextSources.next();
        this.nextTargets.next();
        this.nextAbilities.next();
        this.nextCategories.complete();
        this.nextSegments.complete();
        this.nextSources.complete();
        this.nextTargets.complete();
        this.nextAbilities.complete();
    }

    get categories(): Observable<Array<Category>> {
        return this.categories$.asObservable();
    }

    get segments(): Observable<Array<Segment>> {
        return this.segments$.asObservable()
            .pipe(map(inner_segments => inner_segments.filter(segment => {
                return this.categories$.getValue().filter(category => this.category_filter.has(category.id))
                    .find(category => category.segments.has(segment.id)) !== undefined;
            })));
    }

    get sources(): Observable<Array<EventSource>> {
        return this.sources$.asObservable();
    }

    get targets(): Observable<Array<EventSource>> {
        return this.targets$.asObservable();
    }

    get abilities(): Observable<Array<EventAbility>> {
        return this.abilities$.asObservable();
    }

    get options(): Observable<Array<RaidOption>> {
        return this.options$.asObservable();
    }

    update_category_filter(selected_categories: Array<number>, update_stack: boolean = false, push_history: boolean = true): void {
        if (this.category_filter.size === selected_categories.length && selected_categories.every(elem => this.category_filter.has(elem)))
            return;
        if (update_stack) this.prev_category_filter = this.category_filter;
        else this.prev_category_filter = new Set(selected_categories);
        this.category_filter = new Set(selected_categories);
        this.segments$.next(this.segments$.getValue());

        const new_segment_filter = [];
        const filtered_categories = this.categories$.getValue().filter(category => this.category_filter.has(category.id));
        for (const segment of this.segment_filter)
            if (filtered_categories.find(category => category.segments.has(segment)) !== undefined)
                new_segment_filter.push(segment);
        this.update_segment_filter(new_segment_filter);
        if (update_stack) this.filter_updated$.next(push_history);
    }

    update_segment_filter(selected_segments: Array<number>, update_stack: boolean = false, push_history: boolean = true): void {
        if (this.segment_filter.size === selected_segments.length && selected_segments.every(elem => this.segment_filter.has(elem)))
            return;
        if (update_stack) this.prev_segment_filter = this.segment_filter;
        else this.prev_segment_filter = new Set(selected_segments);
        this.segment_filter = new Set(selected_segments);
        this.current_filtered_intervals = this.segments$.getValue()
            .filter(segment => this.segment_filter.has(segment.id))
            .map(segment => [segment.start_ts, segment.start_ts + segment.duration]);
        this.instanceDataService.set_segment_intervals(this.current_filtered_intervals);
        this.update_filter();
        if (update_stack) this.filter_updated$.next(push_history);
    }

    update_source_filter(selected_sources: Array<number>, update_stack: boolean = false, push_history: boolean = true): void {
        if (this.source_filter.size === selected_sources.length && selected_sources.every(elem => this.source_filter.has(elem)))
            return;
        if (update_stack) this.prev_source_filter = this.source_filter;
        else this.prev_source_filter = new Set(selected_sources);
        this.source_filter = new Set(selected_sources);
        this.instanceDataService.set_source_filter(selected_sources);
        if (update_stack) this.filter_updated$.next(push_history);
    }

    update_target_filter(selected_targets: Array<number>, update_stack: boolean = false, push_history: boolean = true): void {
        if (this.target_filter.size === selected_targets.length && selected_targets.every(elem => this.target_filter.has(elem)))
            return;
        if (update_stack) this.prev_target_filter = this.target_filter;
        else this.prev_target_filter = new Set(selected_targets);
        this.target_filter = new Set(selected_targets);
        this.instanceDataService.set_target_filter(selected_targets);
        if (update_stack) this.filter_updated$.next(push_history);
    }

    update_ability_filter(selected_abilities: Array<number>, update_stack: boolean = false, push_history: boolean = true): void {
        if (this.ability_filter.size === selected_abilities.length && selected_abilities.every(elem => this.ability_filter.has(elem)))
            return;
        if (update_stack) this.prev_ability_filter = this.ability_filter;
        else this.prev_ability_filter = new Set(selected_abilities);
        this.ability_filter = new Set(selected_abilities);
        this.instanceDataService.set_ability_filter(selected_abilities);
        if (update_stack) this.filter_updated$.next(push_history);
    }

    private update_categories(attempts: Array<InstanceViewerAttempt>): void {
        this.nextCategories.next();
        const categories: Map<number, Category> = new Map();
        const non_boss_intervals = this.calculate_non_boss_attempts(attempts);
        categories.set(0, {
            segments: new Set(non_boss_intervals.map((value, index) => -index)),
            id: 0,
            label: "Trash & OOC segments",
            time: non_boss_intervals.reduce((acc, [start, end]) => acc + end - start, 0)
        });

        for (const attempt of attempts) {
            if (categories.has(attempt.encounter_id)) {
                const category = categories.get(attempt.encounter_id);
                category.segments.add(attempt.id);
                category.time += (attempt.end_ts - attempt.start_ts);
                continue;
            }
            categories.set(attempt.encounter_id, {
                segments: new Set([attempt.id]),
                id: attempt.encounter_id,
                label: undefined,
                time: (attempt.end_ts - attempt.start_ts)
            });
        }
        zip(...[...categories.values()].map(category => this.dataService.get_encounter(category.id)
            .pipe(map(encounter => {
                if (!category.label || category.label === CONST_UNKNOWN_LABEL)
                    category.label = !encounter ? CONST_UNKNOWN_LABEL : encounter.localization;
                return category;
            })))).pipe(takeUntil(this.nextCategories.asObservable())).subscribe(update => this.categories$.next(update));
    }

    private update_segments(attempts: Array<InstanceViewerAttempt>): void {
        this.nextSegments.next();
        const segments: Array<Segment> = [];
        for (const attempt of attempts)
            segments.push({
                duration: (attempt.end_ts - attempt.start_ts),
                id: attempt.id,
                encounter_id: attempt.encounter_id,
                is_kill: attempt.is_kill,
                label: undefined,
                start_ts: attempt.start_ts
            });
        for (const [index, start, end] of this.calculate_non_boss_attempts(attempts).map(([i_start, i_end], i_index) => [i_index, i_start, i_end]))
            segments.push({
                duration: (end - start),
                id: -index,
                encounter_id: 0,
                is_kill: false,
                label: "Non-Boss Segment " + (index + 1).toString(),
                start_ts: start
            });
        zip(...[...segments.values()].map(segment => this.dataService.get_encounter(segment.encounter_id)
            .pipe(map(encounter => {
                if (!segment.label || segment.label === CONST_UNKNOWN_LABEL)
                    segment.label = !encounter ? CONST_UNKNOWN_LABEL : encounter.localization;
                return segment;
            })))).pipe(takeUntil(this.nextSegments.asObservable())).subscribe(update => this.segments$.next(update));
    }

    private update_sources(sources: Array<Unit>): void {
        this.nextSources.next();
        const result: Array<Observable<EventSource>> = [];
        // TODO: Optimize!
        for (const source of sources)
            result.push(combineLatest([this.unitService.get_unit_name(source, this.current_meta.end_ts ?? this.current_meta.start_ts),
                this.unitService.is_unit_boss(source)])
                .pipe(map(([label, is_boss]) => {
                    return {
                        id: get_unit_id(source, false),
                        label,
                        is_player: is_player(source, false),
                        is_boss
                    };
                })));
        zip(...result).pipe(takeUntil(this.nextSources.asObservable())).subscribe(update => this.sources$.next(update));
    }

    private update_targets(targets: Array<Unit>): void {
        this.nextTargets.next();
        const result: Array<Observable<EventSource>> = [];
        // TODO: Optimize!
        for (const target of targets)
            result.push(combineLatest([this.unitService.get_unit_name(target, this.current_meta.end_ts ?? this.current_meta.start_ts),
                this.unitService.is_unit_boss(target)])
                .pipe(map(([label, is_boss]) => {
                    return {
                        id: get_unit_id(target, false),
                        label,
                        is_player: is_player(target, false),
                        is_boss
                    };
                })));
        zip(...result).pipe(takeUntil(this.nextTargets.asObservable())).subscribe(update => this.targets$.next(update));
    }

    private update_abilities(abilities: Array<number>): void {
        this.nextAbilities.next();
        const result: Array<Observable<EventAbility>> = [];
        // TODO: Optimize!
        for (const spell_id of abilities)
            result.push(this.spellService.get_localized_basic_spell(spell_id)
                .pipe(map(spell => {
                    return {
                        id: spell_id,
                        label: spell?.localization
                    };
                })));
        zip(...result).pipe(takeUntil(this.nextAbilities.asObservable())).subscribe(update => this.abilities$.next(update));
    }

    private calculate_non_boss_attempts(attempts: Array<InstanceViewerAttempt>): Array<[number, number]> {
        if (!this.current_meta)
            return [];

        let current_start_ts = this.current_meta.start_ts;
        const intervals = [];
        for (const attempt of attempts.sort((left, right) => left.start_ts - right.start_ts)) {
            if (attempt.start_ts > current_start_ts) {
                intervals.push([current_start_ts, attempt.start_ts]);
                current_start_ts = attempt.end_ts;
            } else if (attempt.end_ts > current_start_ts) {
                current_start_ts = attempt.end_ts;
            }
        }
        if (current_start_ts < this.current_meta.end_ts)
            intervals.push([current_start_ts, this.current_meta.end_ts]);
        return intervals;
    }

    private update_filter(): void {
        const sources = new Set();
        for (const [start, end] of this.current_filtered_intervals) {
            for (const source_id of this.current_source_intervals.search(start, end))
                sources.add(source_id);
        }

        const targets = new Set();
        for (const [start, end] of this.current_filtered_intervals) {
            for (const target_id of this.current_target_intervals.search(start, end))
                targets.add(target_id);
        }

        const abilities: Set<number> = new Set();
        for (const [start, end] of this.current_filtered_intervals) {
            for (const ability_id of this.current_ability_intervals.search(start, end))
                if (iterable_some(this.current_ability_map.get(ability_id).values(), (evt_type) => this.current_event_types.has(evt_type)))
                    abilities.add(ability_id);
        }

        this.update_sources(iterable_map(sources.values(), (id) => this.current_source_map.get(id)));
        this.update_targets(iterable_map(targets.values(), (id) => this.current_target_map.get(id)));
        this.update_abilities([...abilities.values()]);
    }

    private async update_subjects(): Promise<void> {
        const source_prom = [];
        source_prom.push([this.instanceDataService.knecht_melee.get_sources(), [12, 1]]);
        source_prom.push([this.instanceDataService.knecht_misc.get_sources(), [3, 10]]);
        source_prom.push([this.instanceDataService.knecht_spell_damage.get_sources(), [13, 1]]);
        source_prom.push([this.instanceDataService.knecht_heal.get_sources(), [14, 1]]);
        source_prom.push([this.instanceDataService.knecht_un_aura.get_sources(), [7, 8, 9]]);
        source_prom.push([this.instanceDataService.knecht_threat.get_sources(), [15]]);
        source_prom.push([this.instanceDataService.knecht_aura.get_sources(), [6]]);

        const target_prom = [];
        target_prom.push([this.instanceDataService.knecht_melee.get_targets(), [12, 1]]);
        target_prom.push([this.instanceDataService.knecht_misc.get_targets(), [3, 10]]);
        target_prom.push([this.instanceDataService.knecht_spell_damage.get_targets(), [13, 1]]);
        target_prom.push([this.instanceDataService.knecht_heal.get_targets(), [14, 1]]);
        target_prom.push([this.instanceDataService.knecht_un_aura.get_targets(), [7, 8, 9]]);
        target_prom.push([this.instanceDataService.knecht_threat.get_targets(), [15]]);
        target_prom.push([this.instanceDataService.knecht_aura.get_targets(), [6]]);

        const ability_prom = [];
        ability_prom.push([this.instanceDataService.knecht_melee.get_abilities(), [12, 1]]);
        ability_prom.push([this.instanceDataService.knecht_misc.get_abilities(), [3, 10]]);
        ability_prom.push([this.instanceDataService.knecht_spell_damage.get_abilities(), [13, 1]]);
        ability_prom.push([this.instanceDataService.knecht_heal.get_abilities(), [14, 1]]);
        ability_prom.push([this.instanceDataService.knecht_un_aura.get_abilities(), [7, 8, 9]]);
        ability_prom.push([this.instanceDataService.knecht_threat.get_abilities(), [15]]);
        ability_prom.push([this.instanceDataService.knecht_aura.get_abilities(), [6]]);

        // Performance: Adding it here initially is extremely expensive!
        const source_intervals = new DataIntervalTree<number>();
        for (const [prom] of source_prom) {
            const subjects = await prom;
            for (const [subject_id, [subject, intervals]] of subjects) {
                if (!this.current_source_map.has(subject_id))
                    this.current_source_map.set(subject_id, subject);
                for (const [start, end] of intervals)
                    source_intervals.insert(start, end, subject_id);
            }
        }
        this.current_source_intervals = source_intervals;

        const target_intervals = new DataIntervalTree<number>();
        for (const [prom] of target_prom) {
            const subjects = await prom;
            for (const [subject_id, [subject, intervals]] of subjects) {
                if (!this.current_target_map.has(subject_id))
                    this.current_target_map.set(subject_id, subject);
                for (const [start, end] of intervals)
                    target_intervals.insert(start, end, subject_id);
            }
        }
        this.current_target_intervals = target_intervals;

        const ability_intervals = new DataIntervalTree<number>();
        for (const [prom, evt_types] of ability_prom) {
            const subjects = await prom;
            for (const [subject_id, [_, intervals]] of subjects) {
                if (this.current_ability_map.has(subject_id)) {
                    const current_evt_types = this.current_ability_map.get(subject_id);
                    for (const evt_type of evt_types) current_evt_types.add(evt_type);
                } else {
                    this.current_ability_map.set(subject_id, new Set(evt_types));
                }
                for (const [start, end] of intervals)
                    ability_intervals.insert(start, end, subject_id);
            }
        }
        this.current_ability_intervals = ability_intervals;
        this.update_filter();
    }

    public select_event_types(evt_types: Array<number>): void {
        if (evt_types.length !== this.current_event_types.size || evt_types.some(evt_type => !this.current_event_types.has(evt_type))) {
            this.current_event_types = new Set(evt_types);
            this.update_filter();
        }
    }

}

