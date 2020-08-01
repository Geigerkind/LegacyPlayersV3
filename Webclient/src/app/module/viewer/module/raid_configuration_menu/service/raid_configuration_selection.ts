import {Injectable} from "@angular/core";
import {Observable, Subject} from "rxjs";
import {RaidConfigurationService} from "./raid_configuration";

@Injectable({
    providedIn: "root",
})
export class RaidConfigurationSelectionService {

    private source_selection$: Subject<Array<number>> = new Subject();

    constructor(
        private raidConfigurationService: RaidConfigurationService
    ) {}

    get source_selection(): Observable<Array<number>> {
        return this.source_selection$.asObservable();
    }

    select_sources(sources: Array<number>): void {
        this.source_selection$.next(sources);
        this.raidConfigurationService.update_source_filter(sources);
    }
}
