import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, combineLatest, Observable, Subject, Subscription, zip} from "rxjs";
import {Category} from "../domain_value/category";
import {InstanceViewerAttempt} from "../../../domain_value/instance_viewer_attempt";
import {Segment} from "../domain_value/segment";
import {EventSource} from "../domain_value/event_source";
import {RaidOption} from "../domain_value/raid_option";
import {map, take, takeUntil} from "rxjs/operators";
import {UnitService} from "../../../service/unit";
import {get_unit_id, is_player, Unit} from "../../../domain_value/unit";
import {InstanceViewerMeta} from "../../../domain_value/instance_viewer_meta";
import {EventAbility} from "../domain_value/event_ability";
import {SpellService} from "../../../service/spell";
import {DataService} from "../../../../../service/data";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class RaidConfigurationService implements OnDestroy {

    private subscription_attempts: Subscription;
    private subscription_sources: Subscription;
    private subscription_targets: Subscription;
    private subscription_abilities: Subscription;
    private subscription_meta: Subscription;

    private current_meta: InstanceViewerMeta;

    private categories$: BehaviorSubject<Array<Category>> = new BehaviorSubject([]);
    private segments$: BehaviorSubject<Array<Segment>> = new BehaviorSubject([]);
    private sources$: BehaviorSubject<Array<EventSource>> = new BehaviorSubject([]);
    private targets$: BehaviorSubject<Array<EventSource>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Array<EventAbility>> = new BehaviorSubject([]);
    private options$: BehaviorSubject<Array<RaidOption>> = new BehaviorSubject([]);

    private category_filter: Set<number> = new Set();
    private segment_filter: Set<number> = new Set();

    private nextCategories: Subject<void> = new Subject();
    private nextSegments: Subject<void> = new Subject();
    private nextSources: Subject<void> = new Subject();
    private nextTargets: Subject<void> = new Subject();
    private nextAbilities: Subject<void> = new Subject();

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService,
        private spellService: SpellService,
        private dataService: DataService
    ) {
        this.subscription_meta = this.instanceDataService.meta.subscribe(meta => {
            this.current_meta = meta;
            this.instanceDataService.attempts.pipe(take(1)).subscribe(attempts => {
                this.update_categories(attempts);
                this.update_segments(attempts);
            });
        });
        this.subscription_attempts = this.instanceDataService.attempts.subscribe(attempts => {
            this.update_categories(attempts);
            this.update_segments(attempts);
        });
        this.subscription_sources = this.instanceDataService.sources.subscribe(sources => this.update_sources(sources));
        this.subscription_targets = this.instanceDataService.targets.subscribe(targets => this.update_targets(targets));
        this.subscription_abilities = this.instanceDataService.abilities.subscribe(abilities => this.update_abilities(abilities));
    }

    ngOnDestroy(): void {
        this.subscription_attempts?.unsubscribe();
        this.subscription_sources?.unsubscribe();
        this.subscription_targets?.unsubscribe();
        this.subscription_abilities?.unsubscribe();
        this.subscription_meta?.unsubscribe();
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

    private set_segment_intervals(): void {
        this.instanceDataService.segment_intervals = this.segments$.getValue()
            .filter(segment => this.segment_filter.has(segment.id))
            .map(segment => [segment.start_ts, segment.start_ts + segment.duration]);
    }

    update_category_filter(selected_categories: Array<number>): void {
        this.category_filter = new Set(selected_categories);
        this.segments$.next(this.segments$.getValue());

        const new_segment_filter = [];
        const filtered_categories = this.categories$.getValue().filter(category => this.category_filter.has(category.id));
        for (const segment of this.segment_filter)
            if (filtered_categories.find(category => category.segments.has(segment)) !== undefined)
                new_segment_filter.push(segment);
        this.update_segment_filter(new_segment_filter);
    }

    update_segment_filter(selected_segments: Array<number>): void {
        this.segment_filter = new Set(selected_segments);
        this.set_segment_intervals();
    }

    update_source_filter(selected_sources: Array<number>): void {
        this.instanceDataService.source_filter = selected_sources;
    }

    update_target_filter(selected_targets: Array<number>): void {
        this.instanceDataService.target_filter = selected_targets;
    }

    update_ability_filter(selected_abilities: Array<number>): void {
        this.instanceDataService.ability_filter = selected_abilities;
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
                if (!category.label)
                    category.label = !encounter ? CONST_UNKNOWN_LABEL : encounter.localization;
                return category;
            })))).pipe(takeUntil(this.nextCategories.asObservable())).subscribe(update => this.categories$.next(update));
    }

    private update_segments(attempts: Array<InstanceViewerAttempt>): void {
        this.nextSegments.next();
        const segments: Array<Segment> = [];
        for (const [index, start, end] of this.calculate_non_boss_attempts(attempts).map(([i_start, i_end], i_index) => [i_index, i_start, i_end]))
            segments.push({
                duration: (end - start),
                id: -index,
                encounter_id: 0,
                is_kill: false,
                label: "Non-Boss Segment " + (index + 1).toString(),
                start_ts: start
            });
        for (const attempt of attempts)
            segments.push({
                duration: (attempt.end_ts - attempt.start_ts),
                id: attempt.id,
                encounter_id: attempt.encounter_id,
                is_kill: attempt.is_kill,
                label: undefined,
                start_ts: attempt.start_ts
            });
        zip(...[...segments.values()].map(segment => this.dataService.get_encounter(segment.encounter_id)
            .pipe(map(encounter => {
                if (!segment.label)
                    segment.label = !encounter ? CONST_UNKNOWN_LABEL : encounter.localization;
                return segment;
            })))).pipe(takeUntil(this.nextSegments.asObservable())).subscribe(update => this.segments$.next(update));
    }

    private update_sources(sources: Array<Unit>): void {
        this.nextSources.next();
        const result: Array<Observable<EventSource>> = [];
        for (const source of sources)
            result.push(combineLatest([this.unitService.get_unit_name(source), this.unitService.is_unit_boss(source)])
                .pipe(map(([label, is_boss]) => {
                    return {
                        id: get_unit_id(source),
                        label,
                        is_player: is_player(source),
                        is_boss
                    };
                })));
        zip(...result).pipe(takeUntil(this.nextSources.asObservable())).subscribe(update => this.sources$.next(update));
    }

    private update_targets(targets: Array<Unit>): void {
        this.nextTargets.next();
        const result: Array<Observable<EventSource>> = [];
        for (const target of targets)
            result.push(combineLatest([this.unitService.get_unit_name(target), this.unitService.is_unit_boss(target)])
                .pipe(map(([label, is_boss]) => {
                    return {
                        id: get_unit_id(target),
                        label,
                        is_player: is_player(target),
                        is_boss
                    };
                })));
        zip(...result).pipe(takeUntil(this.nextTargets.asObservable())).subscribe(update => this.targets$.next(update));
    }

    private update_abilities(abilities: Set<number>): void {
        this.nextAbilities.next();
        const result: Array<Observable<EventAbility>> = [];
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
}
