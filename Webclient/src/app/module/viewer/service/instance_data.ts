import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {BehaviorSubject, Observable, Subject, Subscription} from "rxjs";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";
import {InstanceViewerAttempt} from "../domain_value/instance_viewer_attempt";
import {get_unit_id, Unit} from "../domain_value/unit";
import * as Comlink from "comlink";
import {Remote} from "comlink";
import {Rechenknecht} from "../tool/rechenknecht";
import {auditTime, debounceTime} from "rxjs/operators";
import {KnechtUpdates} from "../domain_value/knecht_updates";
import {LoadingBarService} from "../../../service/loading_bar";

@Injectable({
    providedIn: "root",
})
export class InstanceDataService implements OnDestroy {
    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";
    private static INSTANCE_EXPORT_PARTICIPANTS_URL: string = "/instance/export/participants/:instance_meta_id";
    private static INSTANCE_EXPORT_ATTEMPTS_URL: string = "/instance/export/attempts/:instance_meta_id";

    private subscription: Subscription = new Subscription();

    private instance_meta_id$: number;

    private instance_meta$: BehaviorSubject<InstanceViewerMeta>;
    private participants$: BehaviorSubject<Array<InstanceViewerParticipants>>;
    private attempts$: BehaviorSubject<Array<InstanceViewerAttempt>>;

    private sources$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private targets$: BehaviorSubject<Array<Unit>> = new BehaviorSubject([]);
    private abilities$: BehaviorSubject<Set<number>> = new BehaviorSubject(new Set());

    private attempt_total_duration$: BehaviorSubject<number> = new BehaviorSubject(1);

    private readonly updater: any;

    private worker: Array<Worker> = [];
    knecht_melee: Remote<Rechenknecht>;
    knecht_misc: Remote<Rechenknecht>;
    knecht_replay: Remote<Rechenknecht>;
    knecht_spell_damage: Remote<Rechenknecht>;
    knecht_heal: Remote<Rechenknecht>;
    knecht_dispel: Remote<Rechenknecht>;
    knecht_interrupt: Remote<Rechenknecht>;
    knecht_spell_steal: Remote<Rechenknecht>;
    knecht_threat: Remote<Rechenknecht>;
    knecht_spell_cast: Remote<Rechenknecht>;
    knecht_aura: Remote<Rechenknecht>;

    private public_knecht_updates$: Subject<[Array<KnechtUpdates>, Array<number>]> = new Subject();
    private knecht_updates$: Subject<[KnechtUpdates, Array<number>]> = new Subject();
    private recent_knecht_updates$: [Set<KnechtUpdates>, Set<number>] = [new Set(), new Set()];
    private last_knecht_update$: number = 0;

    constructor(
        private apiService: APIService,
        private loading_bar_service: LoadingBarService
    ) {
        this.updater = setInterval(() => {
            this.load_instance_meta(meta => {
                const current_meta = this.instance_meta$.getValue();
                if (current_meta.end_ts !== meta.end_ts) {
                    this.instance_meta$.next(meta);
                }
            });
            this.load_attempts(attempts => {
                if (attempts.length > this.attempts$.getValue().length) {
                    this.attempts$.next(attempts);
                }
            });
        }, 60000);

        this.subscription.add(this.knecht_updates$.subscribe(([knecht_update, event_types]) => {
            this.recent_knecht_updates$[0].add(knecht_update);
            if (!!event_types) {
                for (const evt_type of event_types)
                    this.recent_knecht_updates$[1].add(evt_type);
            }
            this.last_knecht_update$ = Date.now();
        }));
        this.subscription.add(this.knecht_updates$.pipe(auditTime(250))
            .subscribe(() => {
                if (Date.now() - this.last_knecht_update$ >= 200) {
                    this.public_knecht_updates$.next([[...this.recent_knecht_updates$[0].values()], [...this.recent_knecht_updates$[1].values()]]);
                    this.recent_knecht_updates$[0].clear();
                    this.recent_knecht_updates$[1].clear();
                } else {
                    setTimeout(() => this.knecht_updates$.next([this.recent_knecht_updates$[0][0], []]), 200);
                }
            }));
        this.subscription.add(this.knecht_updates.pipe(auditTime(100)).subscribe(([knecht_updates, evt_types]) => {
            if (knecht_updates.some(elem => [KnechtUpdates.NewData, KnechtUpdates.Initialized, KnechtUpdates.SegmentsChanged].includes(elem)))
                this.update_subjects();
        }));
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
        clearInterval(this.updater);
        this.worker.forEach(worker => worker.terminate());
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

    public set segment_intervals(intervals: Array<[number, number]>) {
        this.attempt_total_duration$.next(intervals.reduce((acc, interval) => acc + interval[1] - interval[0], 0));
        this.knecht_melee.set_segment_intervals(intervals);
        this.knecht_misc.set_segment_intervals(intervals);
        this.knecht_replay.set_segment_intervals(intervals);
        this.knecht_spell_damage.set_segment_intervals(intervals);
        this.knecht_heal.set_segment_intervals(intervals);
        this.knecht_dispel.set_segment_intervals(intervals);
        this.knecht_interrupt.set_segment_intervals(intervals);
        this.knecht_spell_steal.set_segment_intervals(intervals);
        this.knecht_threat.set_segment_intervals(intervals);
        this.knecht_spell_cast.set_segment_intervals(intervals);
        this.knecht_aura.set_segment_intervals(intervals);
    }

    public set source_filter(sources: Array<number>) {
        this.knecht_melee.set_source_filter(sources);
        this.knecht_misc.set_source_filter(sources);
        this.knecht_replay.set_source_filter(sources);
        this.knecht_spell_damage.set_source_filter(sources);
        this.knecht_heal.set_source_filter(sources);
        this.knecht_dispel.set_source_filter(sources);
        this.knecht_interrupt.set_source_filter(sources);
        this.knecht_spell_steal.set_source_filter(sources);
        this.knecht_threat.set_source_filter(sources);
        this.knecht_spell_cast.set_source_filter(sources);
        this.knecht_aura.set_source_filter(sources);
    }

    public set target_filter(targets: Array<number>) {
        this.knecht_melee.set_target_filter(targets);
        this.knecht_misc.set_target_filter(targets);
        this.knecht_replay.set_target_filter(targets);
        this.knecht_spell_damage.set_target_filter(targets);
        this.knecht_heal.set_target_filter(targets);
        this.knecht_dispel.set_target_filter(targets);
        this.knecht_interrupt.set_target_filter(targets);
        this.knecht_spell_steal.set_target_filter(targets);
        this.knecht_threat.set_target_filter(targets);
        this.knecht_spell_cast.set_target_filter(targets);
        this.knecht_aura.set_target_filter(targets);
    }

    public set ability_filter(abilities: Array<number>) {
        this.knecht_melee.set_ability_filter(abilities);
        this.knecht_misc.set_ability_filter(abilities);
        this.knecht_replay.set_ability_filter(abilities);
        this.knecht_spell_damage.set_ability_filter(abilities);
        this.knecht_heal.set_ability_filter(abilities);
        this.knecht_dispel.set_ability_filter(abilities);
        this.knecht_interrupt.set_ability_filter(abilities);
        this.knecht_spell_steal.set_ability_filter(abilities);
        this.knecht_threat.set_ability_filter(abilities);
        this.knecht_spell_cast.set_ability_filter(abilities);
        this.knecht_aura.set_ability_filter(abilities);
    }

    public set instance_meta_id(instance_meta_id: number) {
        if (!!this.instance_meta_id$)
            return;

        this.instance_meta_id$ = instance_meta_id;

        // Cant be put into a function because Angular won't recognize paths otherwise
        const knecht_condition = data => !!data && !!data[0] && data[0] === "KNECHT_UPDATES";
        const handle_loading_bar = data => {
            if (data[1] === KnechtUpdates.WorkStart)
                this.loading_bar_service.incrementCounter();
            else if ([KnechtUpdates.WorkEnd, KnechtUpdates.Initialized].includes(data[1]))
                this.loading_bar_service.decrementCounter();
        };

        for (let i = 0; i < 11; ++i)
            this.loading_bar_service.incrementCounter();

        let worker = new Worker('./../worker/melee.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_melee = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/spell_damage.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);
                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_spell_damage = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/misc.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_misc = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/replay.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_replay = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/dispel.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_dispel = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/heal.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_heal = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/interrupt.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_interrupt = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/spell_cast.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_spell_cast = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/spell_steal.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_spell_steal = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/threat.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_threat = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/aura.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_aura = Comlink.wrap<Rechenknecht>(worker);
    }

    public get meta(): Observable<InstanceViewerMeta> {
        if (!this.instance_meta$) {
            this.instance_meta$ = new BehaviorSubject(undefined);
            this.load_instance_meta(meta => {
                this.instance_meta$.next(meta);
            });
            this.subscription.add(this.instance_meta$.subscribe(meta => {
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
            });
        }
        return this.participants$.asObservable();
    }

    public get attempts(): Observable<Array<InstanceViewerAttempt>> {
        if (!this.attempts$) {
            this.attempts$ = new BehaviorSubject([]);
            this.load_attempts(attempts => {
                this.attempts$.next(attempts);
            });
        }
        return this.attempts$.asObservable();
    }

    public get knecht_updates(): Observable<[Array<KnechtUpdates>, Array<number>]> {
        return this.public_knecht_updates$.asObservable();
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

    private async update_subjects(): Promise<void> {
        // Sources
        const sources_res = new Map();
        const sources = [
            ...await this.knecht_melee.get_sources(),
            ...await this.knecht_misc.get_sources(),
            ...await this.knecht_replay.get_sources(),
            ...await this.knecht_spell_damage.get_sources(),
            ...await this.knecht_heal.get_sources(),
            ...await this.knecht_dispel.get_sources(),
            ...await this.knecht_interrupt.get_sources(),
            ...await this.knecht_spell_steal.get_sources(),
            ...await this.knecht_spell_cast.get_sources(),
            ...await this.knecht_threat.get_sources(),
            ...await this.knecht_aura.get_sources(),
        ];
        for (const source of sources)
            sources_res.set(get_unit_id(source, false), source);

        // Targets
        const targets_res = new Map();
        const targets = [
            ...await this.knecht_melee.get_targets(),
            ...await this.knecht_misc.get_targets(),
            ...await this.knecht_replay.get_targets(),
            ...await this.knecht_spell_damage.get_targets(),
            ...await this.knecht_heal.get_targets(),
            ...await this.knecht_dispel.get_targets(),
            ...await this.knecht_interrupt.get_targets(),
            ...await this.knecht_spell_steal.get_targets(),
            ...await this.knecht_spell_cast.get_targets(),
            ...await this.knecht_threat.get_targets(),
            ...await this.knecht_aura.get_targets(),
        ];
        for (const target of targets)
            targets_res.set(get_unit_id(target, false), target);

        // Abilities
        const abilities_res = new Set<number>();
        const abilities = [
            ...await this.knecht_melee.get_abilities(),
            ...await this.knecht_misc.get_abilities(),
            ...await this.knecht_replay.get_abilities(),
            ...await this.knecht_spell_damage.get_abilities(),
            ...await this.knecht_heal.get_abilities(),
            ...await this.knecht_dispel.get_abilities(),
            ...await this.knecht_interrupt.get_abilities(),
            ...await this.knecht_spell_steal.get_abilities(),
            ...await this.knecht_spell_cast.get_abilities(),
            ...await this.knecht_threat.get_abilities(),
            ...await this.knecht_aura.get_abilities(),
        ];
        for (const ability of abilities)
            abilities_res.add(ability);

        this.abilities$.next(abilities_res);
        this.targets$.next([...targets_res.values()]);
        this.sources$.next([...sources_res.values()]);
    }

}
