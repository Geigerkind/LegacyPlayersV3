import {Injectable, OnDestroy} from "@angular/core";
import {APIService} from "src/app/service/api";
import {Observable, BehaviorSubject, Subject, Subscription} from "rxjs";
import {InstanceViewerMeta} from "../domain_value/instance_viewer_meta";
import {InstanceViewerParticipants} from "../domain_value/instance_viewer_participants";
import {InstanceViewerAttempt} from "../domain_value/instance_viewer_attempt";
import {get_unit_id, Unit} from "../domain_value/unit";
import * as Comlink from "comlink";
import {Rechenknecht} from "../tool/rechenknecht";
import {Remote} from "comlink";
import {debounceTime} from "rxjs/operators";
import {KnechtUpdates} from "../domain_value/knecht_updates";

export enum ChangedSubject {
    SpellCast = 1,
    Death = 2,
    CombatState,
    Loot,
    Position,
    Power,
    AuraApplication,
    Interrupt,
    SpellSteal,
    Dispel,
    ThreatWipe,
    Summon,
    MeleeDamage,
    SpellDamage,
    Heal,
    Threat,
    InstanceMeta,
    Participants,
    Attempts,
    Sources,
    Targets,
    Abilities,
    AttemptTotalDuration
}

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

    private changed$: Subject<ChangedSubject> = new Subject();
    private readonly updater: any;

    private worker: Array<Worker> = [];
    knecht_melee: Remote<Rechenknecht>;
    knecht_spell: Remote<Rechenknecht>;
    knecht_misc: Remote<Rechenknecht>;
    knecht_replay: Remote<Rechenknecht>;

    private knecht_updates$: Subject<KnechtUpdates> = new Subject();

    constructor(
        private apiService: APIService
    ) {
        this.updater = setInterval(() => {
            this.load_instance_meta(meta => {
                const current_meta = this.instance_meta$.getValue();
                if (current_meta.end_ts !== meta.end_ts) {
                    this.instance_meta$.next(meta);
                    this.changed$.next(ChangedSubject.InstanceMeta);
                }
            });
            this.load_attempts(attempts => {
                if (attempts.length > this.attempts$.getValue().length) {
                    this.attempts$.next(attempts);
                    this.changed$.next(ChangedSubject.Attempts);
                }
            });
        }, 60000);

        this.subscription.add(this.knecht_updates.subscribe(knecht_update => {
            if (knecht_update === KnechtUpdates.NewData)
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
        this.knecht_spell.set_segment_intervals(intervals);
        this.knecht_misc.set_segment_intervals(intervals);
        this.knecht_replay.set_segment_intervals(intervals);
    }

    public set source_filter(sources: Array<number>) {
        this.knecht_melee.set_source_filter(sources);
        this.knecht_spell.set_source_filter(sources);
        this.knecht_misc.set_source_filter(sources);
        this.knecht_replay.set_source_filter(sources);
    }

    public set target_filter(targets: Array<number>) {
        this.knecht_melee.set_target_filter(targets);
        this.knecht_spell.set_target_filter(targets);
        this.knecht_misc.set_target_filter(targets);
        this.knecht_replay.set_target_filter(targets);
    }

    public set ability_filter(abilities: Array<number>) {
        this.knecht_melee.set_ability_filter(abilities);
        this.knecht_spell.set_ability_filter(abilities);
        this.knecht_misc.set_ability_filter(abilities);
        this.knecht_replay.set_ability_filter(abilities);
    }

    public set instance_meta_id(instance_meta_id: number) {
        if (!!this.instance_meta_id$)
            return;

        this.instance_meta_id$ = instance_meta_id;

        // Cant be put into a function because Angular won't recognize paths otherwise
        const knecht_condition = data => !!data && !!data[0] && data[0] === "KNECHT_UPDATES";
        let worker = new Worker('./../worker/melee.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data))
                this.knecht_updates$.next(data[1]);
        };
        this.worker.push(worker);
        this.knecht_melee = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/spell.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data))
                this.knecht_updates$.next(data[1]);
        };
        this.worker.push(worker);
        this.knecht_spell = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/misc.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data))
                this.knecht_updates$.next(data[1]);
        };
        this.worker.push(worker);
        this.knecht_misc = Comlink.wrap<Rechenknecht>(worker);

        worker = new Worker('./../worker/replay.worker', {type: 'module'});
        worker.postMessage(["INIT", instance_meta_id]);
        worker.onmessage = ({data}) => {
            if (knecht_condition(data))
                this.knecht_updates$.next(data[1]);
        };
        this.worker.push(worker);
        this.knecht_replay = Comlink.wrap<Rechenknecht>(worker);
    }

    public get meta(): Observable<InstanceViewerMeta> {
        if (!this.instance_meta$) {
            this.instance_meta$ = new BehaviorSubject(undefined);
            this.load_instance_meta(meta => {
                this.instance_meta$.next(meta);
                this.changed$.next(ChangedSubject.InstanceMeta);
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
                this.changed$.next(ChangedSubject.Participants);
            });
        }
        return this.participants$.asObservable();
    }

    public get attempts(): Observable<Array<InstanceViewerAttempt>> {
        if (!this.attempts$) {
            this.attempts$ = new BehaviorSubject([]);
            this.load_attempts(attempts => {
                this.attempts$.next(attempts);
                this.changed$.next(ChangedSubject.Attempts);
            });
        }
        return this.attempts$.asObservable();
    }

    public get knecht_updates(): Observable<KnechtUpdates> {
        return this.knecht_updates$.asObservable().pipe(debounceTime(10));
    }

    public get changed(): Observable<ChangedSubject> {
        return this.changed$.asObservable();
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
            ...await this.knecht_spell.get_sources(),
            ...await this.knecht_misc.get_sources(),
            ...await this.knecht_replay.get_sources()
        ];
        for (const source of sources)
            sources_res.set(get_unit_id(source), source);
        this.sources$.next([...sources_res.values()]);

        // Targets
        const targets_res = new Map();
        const targets = [
            ...await this.knecht_melee.get_targets(),
            ...await this.knecht_spell.get_targets(),
            ...await this.knecht_misc.get_targets(),
            ...await this.knecht_replay.get_targets(),
        ];
        for (const target of targets)
            targets_res.set(get_unit_id(target), target);
        this.targets$.next([...targets_res.values()]);

        // Abilities
        const abilities_res = new Set<number>();
        const abilities = [
            ...await this.knecht_melee.get_abilities(),
            ...await this.knecht_spell.get_abilities(),
            ...await this.knecht_misc.get_abilities(),
            ...await this.knecht_replay.get_abilities(),
        ];
        for (const ability of abilities)
            abilities_res.add(ability);
        this.abilities$.next(abilities_res);
    }

}
