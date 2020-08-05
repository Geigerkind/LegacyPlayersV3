import {Injectable} from "@angular/core";
import {APIService} from "src/app/service/api";
import {BodyColumn} from "../../../../../../../template/table/module/table_body/domain_value/body_column";
import {BehaviorSubject, Observable} from "rxjs";

@Injectable({
    providedIn: "root",
})
export class RankingTableService {
    private static readonly URL_INSTANCE_CHARACTER_RANKING: string = "/instance/ranking/character/:character_id";

    private rows$: BehaviorSubject<Array<Array<BodyColumn>>> = new BehaviorSubject([]);

    constructor(
        private apiService: APIService
    ) {
    }

    get_rows(character_id: number): Observable<Array<Array<BodyColumn>>> {
        this.apiService.get(RankingTableService.URL_INSTANCE_CHARACTER_RANKING.replace(":character_id", character_id.toString()), result => {
            this.rows$.next(result.map(item => {
                const columns: Array<BodyColumn> = [];

                columns.push({
                    type: 0,
                    content: item[0],
                    args: null
                });

                columns.push({
                    type: 1,
                    content: !item[1] ? 0 : (item[1].amount * 1000 / item[1].duration).toFixed(1),
                    args: null
                });

                columns.push({
                    type: 1,
                    content: !item[2] ? 0 : (item[2].amount * 1000 / item[2].duration).toFixed(1),
                    args: null
                });

                columns.push({
                    type: 1,
                    content: !item[3] ? 0 : (item[3].amount * 1000 / item[3].duration).toFixed(1),
                    args: null
                });

                return {
                    color: '',
                    columns
                };
            }));
        });
        return this.rows$.asObservable();
    }

}
