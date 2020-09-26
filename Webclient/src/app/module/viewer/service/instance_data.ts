import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {BehaviorSubject, Observable, Subject, Subscription} from "rxjs";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";
import {InstanceViewerAttempt} from "../domain_value/instance_viewer_attempt";
import * as Comlink from "comlink";
import {Remote} from "comlink";
import {Rechenknecht} from "../tool/rechenknecht";
import {KnechtUpdates} from "../domain_value/knecht_updates";
import {LoadingBarService} from "../../../service/loading_bar";
import {debounceTime} from "rxjs/operators";

@Injectable({
    providedIn: "root",
})
export class InstanceDataService implements OnDestroy {

    private static readonly NUMBER_OF_WORKER: number = 7;

    private static INSTANCE_EXPORT_META_URL: string = "/instance/export/:instance_meta_id";
    private static INSTANCE_EXPORT_PARTICIPANTS_URL: string = "/instance/export/participants/:instance_meta_id";
    private static INSTANCE_EXPORT_ATTEMPTS_URL: string = "/instance/export/attempts/:instance_meta_id";

    private subscription: Subscription = new Subscription();

    private instance_meta_id$: number;

    private instance_meta$: BehaviorSubject<InstanceViewerMeta>;
    private participants$: BehaviorSubject<Array<InstanceViewerParticipants>>;
    private attempts$: BehaviorSubject<Array<InstanceViewerAttempt>>;

    private attempt_total_duration$: BehaviorSubject<number> = new BehaviorSubject(1);

    private readonly updater: any;

    private worker: Array<Worker> = [];
    knecht_melee: Remote<Rechenknecht>;
    knecht_misc: Remote<Rechenknecht>;
    // knecht_replay: Remote<Rechenknecht>;
    knecht_spell_damage: Remote<Rechenknecht>;
    knecht_heal: Remote<Rechenknecht>;
    knecht_un_aura: Remote<Rechenknecht>;
    knecht_threat: Remote<Rechenknecht>;
    // knecht_spell_cast: Remote<Rechenknecht>;
    knecht_aura: Remote<Rechenknecht>;

    private public_knecht_updates$: Subject<[Array<KnechtUpdates>, Array<number>]> = new Subject();
    private knecht_updates$: Subject<[KnechtUpdates, Array<number>]> = new Subject();
    private recent_knecht_updates$: [Set<KnechtUpdates>, Set<number>] = [new Set(), new Set()];
    private worker_initialized: number = 0;

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

        this.subscription.add(this.knecht_updates$
            .subscribe(([knecht_update, event_types]) => {
                if (knecht_update === KnechtUpdates.Initialized) {
                    ++this.worker_initialized;
                }

                this.recent_knecht_updates$[0].add(knecht_update);
                if (!!event_types) {
                    for (const evt_type of event_types)
                        this.recent_knecht_updates$[1].add(evt_type);
                }
            }));
        this.subscription.add(this.knecht_updates$.pipe(debounceTime(50)).subscribe(() => {
            if (this.worker_initialized < InstanceDataService.NUMBER_OF_WORKER)
                return;

            this.public_knecht_updates$.next([[...this.recent_knecht_updates$[0].values()], [...this.recent_knecht_updates$[1].values()]]);
            this.recent_knecht_updates$[0].clear();
            this.recent_knecht_updates$[1].clear();
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

    public async set_segment_intervals(intervals: Array<[number, number]>) {
        this.attempt_total_duration$.next(intervals.reduce((acc, interval) => acc + interval[1] - interval[0], 0));

        const promisses = [];
        promisses.push(this.knecht_melee.set_segment_intervals(intervals));
        promisses.push(this.knecht_misc.set_segment_intervals(intervals));
        // promisses.push(this.knecht_replay.set_segment_intervals(intervals));
        promisses.push(this.knecht_spell_damage.set_segment_intervals(intervals));
        promisses.push(this.knecht_heal.set_segment_intervals(intervals));
        promisses.push(this.knecht_un_aura.set_segment_intervals(intervals));
        promisses.push(this.knecht_threat.set_segment_intervals(intervals));
        // promisses.push(this.knecht_spell_cast.set_segment_intervals(intervals));
        promisses.push(this.knecht_aura.set_segment_intervals(intervals));

        for (const prom of promisses)
            await prom;
        this.knecht_updates$.next([KnechtUpdates.FilterChanged, []]);
    }

    public async set_source_filter(sources: Array<number>) {
        const promisses = [];
        promisses.push(this.knecht_melee.set_source_filter(sources));
        promisses.push(this.knecht_misc.set_source_filter(sources));
        // promisses.push(this.knecht_replay.set_source_filter(sources));
        promisses.push(this.knecht_spell_damage.set_source_filter(sources));
        promisses.push(this.knecht_heal.set_source_filter(sources));
        promisses.push(this.knecht_un_aura.set_source_filter(sources));
        promisses.push(this.knecht_threat.set_source_filter(sources));
        // promisses.push(this.knecht_spell_cast.set_source_filter(sources));
        promisses.push(this.knecht_aura.set_source_filter(sources));

        for (const prom of promisses)
            await prom;
        this.knecht_updates$.next([KnechtUpdates.FilterChanged, []]);
    }

    public async set_target_filter(targets: Array<number>) {
        const promisses = [];
        promisses.push(this.knecht_melee.set_target_filter(targets));
        promisses.push(this.knecht_misc.set_target_filter(targets));
        // promisses.push(this.knecht_replay.set_target_filter(targets));
        promisses.push(this.knecht_spell_damage.set_target_filter(targets));
        promisses.push(this.knecht_heal.set_target_filter(targets));
        promisses.push(this.knecht_un_aura.set_target_filter(targets));
        promisses.push(this.knecht_threat.set_target_filter(targets));
        // promisses.push(this.knecht_spell_cast.set_target_filter(targets));
        promisses.push(this.knecht_aura.set_target_filter(targets));

        for (const prom of promisses)
            await prom;
        this.knecht_updates$.next([KnechtUpdates.FilterChanged, []]);
    }

    public async set_ability_filter(abilities: Array<number>) {
        const promisses = [];
        promisses.push(this.knecht_melee.set_ability_filter(abilities));
        promisses.push(this.knecht_misc.set_ability_filter(abilities));
        // promisses.push(this.knecht_replay.set_ability_filter(abilities));
        promisses.push(this.knecht_spell_damage.set_ability_filter(abilities));
        promisses.push(this.knecht_heal.set_ability_filter(abilities));
        promisses.push(this.knecht_un_aura.set_ability_filter(abilities));
        promisses.push(this.knecht_threat.set_ability_filter(abilities));
        // promisses.push(this.knecht_spell_cast.set_ability_filter(abilities));
        promisses.push(this.knecht_aura.set_ability_filter(abilities));

        for (const prom of promisses)
            await prom;
        this.knecht_updates$.next([KnechtUpdates.FilterChanged, []]);
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

        for (let i = 0; i < InstanceDataService.NUMBER_OF_WORKER; ++i)
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

        /*
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
         */

        worker = new Worker('./../worker/un_aura.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data)) {
                handle_loading_bar(data);

                this.knecht_updates$.next([data[1], data[2]]);
            }
        };
        this.worker.push(worker);
        this.knecht_un_aura = Comlink.wrap<Rechenknecht>(worker);

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

        /*
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
         */

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

    public get attempt_total_duration(): Observable<number> {
        return this.attempt_total_duration$.asObservable();
    }

}
