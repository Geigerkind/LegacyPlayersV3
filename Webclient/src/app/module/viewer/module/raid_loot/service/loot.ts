import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, from, Observable, of, Subscription} from "rxjs";
import {Loot} from "../domain_value/loot";
import {concatMap, map, take} from "rxjs/operators";
import {InstanceViewerAttempt} from "../../../domain_value/instance_viewer_attempt";
import {Event} from "../../../domain_value/event";
import {LootItem} from "../domain_value/loot_item";
import {Player} from "../../../domain_value/player";
import {UnitService} from "../../../service/unit";
import {DataService} from "../../../../../service/data";
import {InstanceViewerMeta} from "../../../domain_value/instance_viewer_meta";
import {Localized} from "../../../../../domain_value/localized";
import {BasicItem} from "../../../../../domain_value/data/basic_item";
import {Loot as ViewerLoot} from "../../../domain_value/loot";
import {CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {KnechtUpdates} from "../../../domain_value/knecht_updates";

@Injectable({
    providedIn: "root",
})
export class LootService implements OnDestroy {

    private subscription: Subscription;
    private current_meta: InstanceViewerMeta;

    private loot$: BehaviorSubject<Array<Loot>> = new BehaviorSubject([]);

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService,
        private dataService: DataService
    ) {
        this.subscription = this.instanceDataService.meta.subscribe(meta => {
            this.current_meta = meta;
            this.reload();
        });
        this.instanceDataService.knecht_updates.subscribe(async knecht_update => {
            if ([KnechtUpdates.NewData, KnechtUpdates.FilterChanged].includes(knecht_update))
                this.reload();
        });
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get loot(): Observable<Array<Loot>> {
        return this.loot$.asObservable();
    }

    private reload() {
        if (!this.current_meta)
            return;
        this.instanceDataService
            .attempts.pipe(take(1))
            .subscribe(attempts => {
                from(this.instanceDataService.knecht_misc.get_loot()).pipe(take(1))
                    .subscribe( loot => {
                        this.loot$.next(this.create_loot(attempts, loot));
                    });
            });
    }

    private get_basic_item(item_id: number): Observable<Localized<BasicItem> | undefined> {
        return this.dataService
            .get_server_by_id(this.current_meta.server_id)
            .pipe(concatMap(server => {
                return !server ? of(undefined) : this.dataService
                    .get_localized_basic_item(server.expansion_id, item_id);
            }));
    }

    private create_loot(attempts: Array<InstanceViewerAttempt>, loot: Array<Event>): Array<Loot> {
        const result = new Map<number, Loot>();
        const sorted_attempts = attempts.sort((left, right) => left.start_ts - right.start_ts);
        for (const item of loot) {
            let last_attempt: InstanceViewerAttempt;
            for (const attempt of sorted_attempts) {
                if (attempt.start_ts > item.timestamp)
                    break;
                last_attempt = attempt;
            }
            const loot_item: LootItem = {
                receiver_id: ((item.subject as any).Player as Player).character_id,
                receiver: this.unitService.get_unit_name(item.subject),
                item: this.get_basic_item(((item.event as any).Loot as ViewerLoot).item_id),
                amount: ((item.event as any).Loot as ViewerLoot).amount
            };
            if (!last_attempt) {
                if (result.has(0)) {
                    const entry = result.get(0);
                    entry.loot_items.push(loot_item);
                } else  {
                    result.set(0, {
                        name: of(CONST_UNKNOWN_LABEL),
                        loot_items: [loot_item]
                    });
                }
            } else {
                if (result.has(last_attempt.encounter_id)) {
                    const entry = result.get(last_attempt.encounter_id);
                    entry.loot_items.push(loot_item);
                } else {
                    result.set(last_attempt.encounter_id, {
                        name: this.dataService.get_encounter(last_attempt.encounter_id).pipe(map(encounter => encounter?.localization)),
                        loot_items: [loot_item]
                    });
                }
            }
        }
        return [...result.values()];
    }
}
