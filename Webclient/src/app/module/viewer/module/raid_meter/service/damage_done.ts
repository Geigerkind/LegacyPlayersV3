import {Injectable, OnDestroy} from "@angular/core";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {get_unit_id} from "../../../domain_value/unit";
import {UtilService} from "./util";
import {SpellCast} from "../../../domain_value/spell_cast";
import {group_by} from "../../../../../stdlib/group_by";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";

@Injectable({
    providedIn: "root",
})
export class DamageDoneService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private newData: Map<number, Map<number, number>>;
    private initialized: boolean = false;

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
        this.instanceDataService.spell_casts.pipe(take(1)).subscribe(spell_casts => {
            this.spell_casts = spell_casts;
            this.commit();
        });
        this.instanceDataService.spell_damage.pipe(take(1)).subscribe(spell_damage => {
            this.spell_damage = spell_damage;
            this.commit();
        });
        this.instanceDataService.melee_damage.pipe(take(1)).subscribe(melee_damage => {
            this.melee_damage = melee_damage;
            this.commit();
        });

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellCast].includes(changed))
                this.instanceDataService.spell_casts.pipe(take(1)).subscribe(spell_casts => {
                    this.spell_casts = spell_casts;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellDamage].includes(changed))
                this.instanceDataService.spell_damage.pipe(take(1)).subscribe(spell_damage => {
                    this.spell_damage = spell_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.MeleeDamage].includes(changed))
                this.instanceDataService.melee_damage.pipe(take(1)).subscribe(melee_damage => {
                    this.melee_damage = melee_damage;
                    this.commit();
                });
        });
    }

    private commit(): void {
        this.newData = new Map();

        // Melee Damage
        let grouping = group_by([...this.melee_damage.values()], (event) => get_unit_id(event.subject));
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
        grouping = group_by(this.spell_damage, (event) => get_unit_id(event.subject));
        // tslint:disable-next-line:forin
        for (const unit_id in grouping) {
            const subject_id = Number(unit_id);
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(grouping[unit_id][0].subject));
            if (!this.newData.has(subject_id))
                this.newData.set(subject_id, new Map());

            grouping[subject_id].forEach(event => this.feed_spell_damage(this.newData.get(subject_id), event));
        }

        this.data$.next([...this.newData.entries()]
            .map(([unit_id, abilities]) => [unit_id, [...abilities.entries()]]));
    }

    private feed_spell_damage(abilities_data: Map<number, number>, event: Event): void {
        const spell_cast_id = ((event.event as any).SpellDamage).spell_cast_id as number;
        const spell_cast_event = this.spell_casts.get(spell_cast_id);
        if (!spell_cast_event)
            return;
        const damage = (event.event as any).SpellDamage.damage.damage;
        const spell_cast = (spell_cast_event.event as any).SpellCast as SpellCast;
        if (!this.abilities$.has(spell_cast.spell_id))
            this.abilities$.set(spell_cast.spell_id, this.utilService.get_row_ability_subject(spell_cast.spell_id));

        if (abilities_data.has(spell_cast.spell_id)) abilities_data.set(spell_cast.spell_id, abilities_data.get(spell_cast.spell_id) + damage);
        else abilities_data.set(spell_cast.spell_id, damage);
    }

}
