import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {Observable, Subject} from "rxjs";
import {SpeedRun} from "../domain_value/speed_run";

@Injectable({
    providedIn: "root",
})
export class SpeedRunService {
    private static readonly URL_INSTANCE_SPEED_RUN: string = "/instance/speed_run";

    private speed_runs$: Subject<Array<SpeedRun>> = new Subject();
    private current_mode$: number = 1;
    private current_map_id$: number = 1;
    private current_server_ids$: Array<number> = [];

    private speed_runs_internal: Array<SpeedRun> = [];

    constructor(
        private apiService: APIService
    ) {
        this.apiService.get(SpeedRunService.URL_INSTANCE_SPEED_RUN, result => {
            this.speed_runs_internal = result;
            this.commit();
        });
    }

    get speed_runs(): Observable<Array<SpeedRun>> {
        return this.speed_runs$.asObservable();
    }

    select(mode: number, map_id: number, server_ids: Array<number>): void {
        this.current_mode$ = mode;
        this.current_map_id$ = map_id;
        this.current_server_ids$ = server_ids;
        this.commit();
    }

    commit(): void {
        const result = this.speed_runs_internal.filter(speed_run => speed_run.map_id === this.current_map_id$ && this.current_server_ids$.includes(speed_run.server_id))
            .sort((left, right) => right.duration - left.duration);
        this.speed_runs$.next(result);
    }

}
