import {Injectable} from "@angular/core";
import {Observable, Subject} from "rxjs";
import {APIService} from "../../../../../service/api";
import {SpeedKill} from "../domain_value/speed_kill";

@Injectable({
    providedIn: "root",
})
export class SpeedKillService {
    private static readonly URL_INSTANCE_SPEED_KILL: string = "/instance/speed_kill";

    private speed_kills$: Subject<Array<SpeedKill>> = new Subject();
    private current_mode$: number = 1;
    private current_encounter_id$: number = 1;
    private current_server_ids$: Array<number> = [];
    private current_difficulty_ids$: Array<number> = [];

    private speed_kills_internal: Array<SpeedKill> = [];

    constructor(
        private apiService: APIService
    ) {
        this.apiService.get(SpeedKillService.URL_INSTANCE_SPEED_KILL, result => {
            this.speed_kills_internal = result;
            this.commit();
        });
    }

    get speed_kills(): Observable<Array<SpeedKill>> {
        return this.speed_kills$.asObservable();
    }

    select(mode: number, encounter_id: number, server_ids: Array<number>, difficulty_ids: Array<number>): void {
        this.current_mode$ = mode;
        this.current_encounter_id$ = encounter_id;
        this.current_server_ids$ = server_ids;
        this.current_difficulty_ids$ = difficulty_ids;
        this.commit();
    }

    commit(): void {
        const result = this.speed_kills_internal.filter(speed_kill => speed_kill.encounter_id === this.current_encounter_id$
            && this.current_server_ids$.includes(speed_kill.server_id) && this.current_difficulty_ids$.includes(speed_kill.difficulty_id))
            .sort((left, right) => left.duration - right.duration);
        this.speed_kills$.next(result);
    }
}
