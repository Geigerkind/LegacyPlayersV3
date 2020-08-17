import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {take} from "rxjs/operators";
import {commit_dispel} from "../stdlib/dispel";
import {te_dispel} from "../../../extractor/targets";

@Injectable({
    providedIn: "root",
})
export class DispelReceivedService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<UnAuraOverviewRow>]>> = new BehaviorSubject([]);
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private dispels: Array<Event> = [];

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get_data(units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<UnAuraOverviewRow>]>> {
        if (!this.initialized) {
            this.units$ = units;
            this.initialize();
        }
        return this.data$.asObservable();
    }

    private initialize(): void {
        this.initialized = true;
        this.instanceDataService.get_aura_applications().pipe(take(1)).subscribe(aura_applications => {
            this.aura_applications = aura_applications;
            this.commit();
        });
        this.instanceDataService.get_spell_casts(true).pipe(take(1)).subscribe(spell_casts => {
            this.spell_casts = spell_casts;
            this.commit();
        });
        this.instanceDataService.get_dispels(true).pipe(take(1)).subscribe(dispels => {
            this.dispels = dispels;
            this.commit();
        });

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.AuraApplication].includes(changed))
                this.instanceDataService.get_aura_applications().pipe(take(1)).subscribe(aura_applications => {
                    this.aura_applications = aura_applications;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellCast].includes(changed))
                this.instanceDataService.get_spell_casts(true).pipe(take(1)).subscribe(spell_casts => {
                    this.spell_casts = spell_casts;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.Dispel].includes(changed))
                this.instanceDataService.get_dispels(true).pipe(take(1)).subscribe(dispels => {
                    this.dispels = dispels;
                    this.commit();
                });
        });
    }

    private commit(): void {
        commit_dispel(this.utilService, this.data$, this.units$,
            this.spell_casts, this.aura_applications, this.dispels, te_dispel(this.aura_applications));
    }
}
