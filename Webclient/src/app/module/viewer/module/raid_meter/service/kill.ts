import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {take} from "rxjs/operators";
import {commit_death} from "../stdlib/death";
import {te_death, te_melee_damage, te_spell_damage} from "../../../extractor/targets";

@Injectable({
    providedIn: "root",
})
export class KillService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<DeathOverviewRow>]>> = new BehaviorSubject([]);
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private spell_damage: Array<Event> = [];
    private melee_damage: Map<number, Event> = new Map();
    private deaths: Array<Event> = [];

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get_data(units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<DeathOverviewRow>]>> {
        if (!this.initialized) {
            this.units$ = units;
            this.initialize();
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
        this.instanceDataService.get_spell_damage().pipe(take(1)).subscribe(spell_damage => {
            this.spell_damage = spell_damage;
            this.commit();
        });
        this.instanceDataService.get_melee_damage().pipe(take(1)).subscribe(melee_damage => {
            this.melee_damage = melee_damage;
            this.commit();
        });
        this.instanceDataService.get_deaths(true).pipe(take(1)).subscribe(deaths => {
            this.deaths = deaths;
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
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellDamage].includes(changed))
                this.instanceDataService.get_spell_damage().pipe(take(1)).subscribe(spell_damage => {
                    this.spell_damage = spell_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.MeleeDamage].includes(changed))
                this.instanceDataService.get_melee_damage().pipe(take(1)).subscribe(melee_damage => {
                    this.melee_damage = melee_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.Death].includes(changed))
                this.instanceDataService.get_deaths(true).pipe(take(1)).subscribe(deaths => {
                    this.deaths = deaths;
                    this.commit();
                });
        });
    }

    private commit(): void {
        commit_death(this.utilService, this.data$, this.units$, this.melee_damage,
            this.spell_casts, this.aura_applications, this.spell_damage, this.deaths, te_melee_damage,
            te_spell_damage, te_death);
    }
}
