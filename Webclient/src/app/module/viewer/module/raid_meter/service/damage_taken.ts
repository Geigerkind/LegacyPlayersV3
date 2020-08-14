import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {take} from "rxjs/operators";
import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id} from "../../../domain_value/unit";
import {ce_spell_damage} from "../../../extractor/causes";
import {te_melee_damage, te_spell_damage} from "../../../extractor/targets";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";
import {get_spell_damage} from "../../../extractor/events";

@Injectable({
    providedIn: "root",
})
export class DamageTakenService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private newData: Map<number, Map<number, number>>;
    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private spell_damage: Array<Event> = [];
    private melee_damage: Map<number, Event> = new Map();

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get_data(abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<[number, number]>]>> {
        if (!this.initialized) {
            this.abilities$ = abilities;
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
        this.instanceDataService.get_spell_damage(true).pipe(take(1)).subscribe(spell_damage => {
            this.spell_damage = spell_damage;
            this.commit();
        });
        this.instanceDataService.get_melee_damage(true).pipe(take(1)).subscribe(melee_damage => {
            this.melee_damage = melee_damage;
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
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellDamage].includes(changed))
                this.instanceDataService.get_spell_damage(true).pipe(take(1)).subscribe(spell_damage => {
                    this.spell_damage = spell_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.MeleeDamage].includes(changed))
                this.instanceDataService.get_melee_damage(true).pipe(take(1)).subscribe(melee_damage => {
                    this.melee_damage = melee_damage;
                    this.commit();
                });
        });
    }

    private commit(): void {
        this.newData = new Map();

        // Melee Damage
        let grouping = group_by([...this.melee_damage.values()], (event) => get_unit_id(te_melee_damage(event)));
        // @ts-ignore
        // tslint:disable-next-line:forin
        for (const unit_id: number in grouping) {
            const subject_id = Number(unit_id);

            if (!this.abilities$.has(0))
                this.abilities$.set(0, this.utilService.get_row_ability_subject_auto_attack());
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(grouping[unit_id][0].subject));

            const total_damage = grouping[unit_id].reduce((acc, event) => acc + (event.event as any).MeleeDamage.damage, 0);
            if (!this.newData.has(subject_id))
                this.newData.set(subject_id, new Map([[0, total_damage]]));
            else this.newData.get(subject_id).set(0, total_damage);
        }

        // Spell Damage
        grouping = group_by(this.spell_damage, (event) => get_unit_id(te_spell_damage(event)));
        // tslint:disable-next-line:forin
        for (const unit_id in grouping) {
            const subject_id = Number(unit_id);
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(te_spell_damage(grouping[unit_id][0])));
            if (!this.newData.has(subject_id))
                this.newData.set(subject_id, new Map());

            grouping[subject_id].forEach(event => this.feed_spell_damage(this.newData.get(subject_id), event));
        }

        this.data$.next([...this.newData.entries()]
            .map(([unit_id, abilities]) => [unit_id, [...abilities.entries()]]));
    }

    private feed_spell_damage(abilities_data: Map<number, number>, event: Event): void {
        const spell_id = ae_spell_cast_or_aura_application(ce_spell_damage, this.spell_casts, this.aura_applications)(event)[0];
        if (!spell_id)
            return;
        const damage = get_spell_damage(event).damage.damage;
        if (!this.abilities$.has(spell_id))
            this.abilities$.set(spell_id, this.utilService.get_row_ability_subject(spell_id));

        if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + damage);
        else abilities_data.set(spell_id, damage);
    }
}
