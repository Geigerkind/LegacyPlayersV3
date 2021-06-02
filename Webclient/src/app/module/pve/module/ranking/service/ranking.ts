import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {BehaviorSubject, Observable, Subject} from "rxjs";
import {RankingResult} from "../domain_value/ranking_result";
import {RankingCharacterMeta} from "../domain_value/ranking_character_meta";
import {RankingRow} from "../domain_value/ranking_row";
import {DataService} from "../../../../../service/data";

@Injectable({
    providedIn: "root",
})
export class RankingService {
    private static readonly URL_INSTANCE_RANKING_DPS: string = "/instance/ranking/dps";
    private static readonly URL_INSTANCE_RANKING_HPS: string = "/instance/ranking/hps";
    private static readonly URL_INSTANCE_RANKING_TPS: string = "/instance/ranking/tps";
    private static readonly URL_INSTANCE_ATTEMPT_DELETE: string = "/instance/ranking/unrank";

    private rankings$: BehaviorSubject<Array<RankingRow>> = new BehaviorSubject([]);
    private dps_rankings$: Subject<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> = new Subject();
    private hps_rankings$: Subject<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> = new Subject();
    private tps_rankings$: Subject<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> = new Subject();

    private dps_rankings: Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]> = [];
    private hps_rankings: Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]> = [];
    private tps_rankings: Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]> = [];

    private current_mode$: number = 1;
    private current_selection$: number = 1;
    private current_encounter_ids$: Array<number> = [];
    private current_specs$: Array<[number, number]> = [];
    private current_server_ids$: Array<number> = [];
    private current_difficulty_ids$: Array<number> = [];
    private current_season_ids$: Array<number> = [];

    constructor(
        private apiService: APIService,
        private dataService: DataService
    ) {
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_DPS, result => {
            this.dps_rankings = result;
            this.dps_rankings$.next(result);
            this.commit();
        });
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_HPS, result => {
            this.hps_rankings = result;
            this.hps_rankings$.next(result);
            this.commit();
        });
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_TPS, result => {
            this.tps_rankings = result;
            this.tps_rankings$.next(result);
            this.commit();
        });
    }

    get rankings(): Observable<Array<RankingRow>> {
        return this.rankings$.asObservable();
    }

    get all_dps_rankings(): Observable<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> {
        return this.dps_rankings$.asObservable();
    }

    get all_hps_rankings(): Observable<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> {
        return this.hps_rankings$.asObservable();
    }

    get all_tps_rankings(): Observable<Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]>> {
        return this.tps_rankings$.asObservable();
    }

    select(mode: number, selection: number, encounter_ids: Array<number>, spec_ids: Array<number>, server_ids: Array<number>, difficulty_ids: Array<number>, season_ids: Array<number>): void {
        this.current_selection$ = selection;
        this.current_encounter_ids$ = encounter_ids;
        this.current_specs$ = spec_ids.map(spec_id => [this.dataService.spec_mapping.get(spec_id)[0], this.dataService.spec_mapping.get(spec_id)[1]]);
        this.current_server_ids$ = server_ids;
        this.current_difficulty_ids$ = difficulty_ids;
        this.current_season_ids$ = season_ids;
        if (mode !== this.current_mode$)
            this.load_current_mode();
        else
            this.commit();
        this.current_mode$ = mode;
    }

    private commit() {
        const new_rankings = new Map<number, [RankingCharacterMeta, Array<Array<number>>]>();
        if (this.current_selection$ === 1) {
            this.current_mode_data
                .filter(([npc_id, char_rankings]) => this.current_encounter_ids$.includes(npc_id))
                .forEach(([npc_id, char_rankings]) =>
                    char_rankings
                        .filter(([character_id, meta, rankings]) => this.current_server_ids$.includes(meta.server_id)
                            && !!this.current_specs$.find(spec => spec[0] === meta.hero_class_id))
                        .map(([character_id, meta, rankings]) => {
                            const best_result = rankings
                                .filter(ranking => this.current_difficulty_ids$.includes(ranking.difficulty_id) && this.current_season_ids$.includes(ranking.season_index)
                                    && !!this.current_specs$.find(spec => spec[0] === meta.hero_class_id && spec[1] === ranking.character_spec))
                                .reduce((best, ranking) => {
                                    const ranking_result = (ranking.amount * 1000) / ranking.duration;
                                    return best[0] > ranking_result ? best : [ranking_result, ranking.instance_meta_id, ranking.attempt_id, ranking.character_spec, npc_id, ranking.amount, ranking.duration];
                                }, [0, 0, 0]);
                            if (!new_rankings.has(character_id))
                                new_rankings.set(character_id, [meta, []]);
                            new_rankings.get(character_id)[1].push(best_result);
                        })
                );
        }
        this.rankings$.next([...new_rankings.entries()]
            .filter(([_character_id, [_character_meta, amounts]]) =>
                amounts.filter(am => am[0] > 0).length === this.current_encounter_ids$.length)
            .map(([character_id, [character_meta, ranking_results]]) => {
                const result = ranking_results.reduce(([count, acc], amount) => [++count, acc + amount[0]], [0, 0]);
                return {
                    character_id,
                    character_meta,
                    amount: Number((result[1] / result[0])),
                    instance_meta_ids: ranking_results.map(rr => rr[1]),
                    attempt_ids: ranking_results.map(rr => rr[2]),
                    spec_ids: ranking_results.map(rr => rr[3]),
                    encounter_ids: ranking_results.map(rr => rr[4]),
                    amounts: ranking_results.map(rr => rr[5]),
                    durations: ranking_results.map(rr => rr[6]),
                };
            }).sort((left, right) => right.amount - left.amount));
    }

    delete(attempt_id: number): void {
        this.apiService.delete(RankingService.URL_INSTANCE_ATTEMPT_DELETE, attempt_id, () => {
            this.load_current_mode();
        });
    }

    private get current_mode_data(): Array<[number, Array<[number, RankingCharacterMeta, Array<RankingResult>]>]> {
        if (this.current_mode$ === 1)
            return this.dps_rankings;
        if (this.current_mode$ === 2)
            return this.hps_rankings;
        return this.tps_rankings;
    }

    private load_current_mode(): void {
        if (this.current_mode$ === 1)
            this.load_dps();
        else if (this.current_mode$ === 2)
            this.load_hps();
        else this.load_tps();
    }

    private load_dps(): void {
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_DPS, result => {
            this.dps_rankings = result;
            this.dps_rankings$.next(result);
            this.commit();
        });
    }

    private load_hps(): void {
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_HPS, result => {
            this.hps_rankings = result;
            this.hps_rankings$.next(result);
            this.commit();
        });
    }

    private load_tps(): void {
        this.apiService.get(RankingService.URL_INSTANCE_RANKING_TPS, result => {
            this.tps_rankings = result;
            this.tps_rankings$.next(result);
            this.commit();
        });
    }
}
