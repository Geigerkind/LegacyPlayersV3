import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {take} from "rxjs/operators";
import {se_aura_app_or_own} from "../../../extractor/sources";
import {ce_heal} from "../../../extractor/causes";
import {commit_heal} from "../stdlib/heal";
import {HealMode} from "../../../domain_value/heal_mode";

@Injectable({
    providedIn: "root",
})
export class HealDoneService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private heal: Array<Event> = [];

    private current_mode: HealMode = HealMode.Total;

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get_data(mode: HealMode, abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<[number, number]>]>> {
        if (!this.initialized) {
            this.abilities$ = abilities;
            this.units$ = units;
            this.current_mode = mode;
            this.initialize();
        } else if (this.current_mode !== mode) {
            this.current_mode = mode;
            this.commit();
        }
        return this.data$.asObservable();
    }

    private initialize(): void {
        this.initialized = true;
        this.instanceDataService.get_aura_applications(true).pipe(take(1)).subscribe(aura_applications => {
            this.aura_applications = aura_applications;
            this.commit();
        });
        this.instanceDataService.get_spell_casts().pipe(take(1)).subscribe(spell_casts => {
            this.spell_casts = spell_casts;
            this.commit();
        });
        this.instanceDataService.get_heal().pipe(take(1)).subscribe(heal => {
            this.heal = heal;
            this.commit();
        });

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.AuraApplication].includes(changed))
                this.instanceDataService.get_aura_applications(true).pipe(take(1)).subscribe(aura_applications => {
                    this.aura_applications = aura_applications;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellCast].includes(changed))
                this.instanceDataService.get_spell_casts().pipe(take(1)).subscribe(spell_casts => {
                    this.spell_casts = spell_casts;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.Heal].includes(changed))
                this.instanceDataService.get_heal().pipe(take(1)).subscribe(heal => {
                    this.heal = heal;
                    this.commit();
                });
        });
    }

    private commit(): void {
        commit_heal(this.current_mode, this.utilService, this.data$, this.abilities$, this.units$,
            this.spell_casts, this.aura_applications, this.heal, se_aura_app_or_own(ce_heal, this.aura_applications));
    }
}
