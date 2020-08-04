import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {BehaviorSubject, Observable} from "rxjs";

@Injectable({
    providedIn: "root",
})
export class RankingService {

    private rankings$: BehaviorSubject<Array<[number, number]>> = new BehaviorSubject([]);

    constructor(
        private apiService: APIService
    ) {
    }

    get rankings(): Observable<Array<[number, number]>> {
        return this.rankings$.asObservable();
    }

}
