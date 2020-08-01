import {Injectable} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable} from "rxjs";
import {RaidMeterRow} from "../domain_value/raid_meter_row";
import {map, take} from "rxjs/operators";
import {EventType} from "../../../domain_value/event_type";
import {MeleeDamage} from "../../../domain_value/melee_damage";
import {SpellDamage} from "../../../domain_value/spell_damage";
import {Damage} from "../../../domain_value/damage";
import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {UtilService} from "./util";
import {SpellCast} from "../../../domain_value/spell_cast";

@Injectable({
    providedIn: "root",
})
export class DamageTakenService {

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    get rows(): Observable<Array<RaidMeterRow>> {
        return this.rows$.asObservable()
            .pipe(map(rows => rows.sort((left, right) => right.amount - left.amount)));
    }

    private rows$: BehaviorSubject<Array<RaidMeterRow>> = new BehaviorSubject([]);
    private newRows: Map<number, RaidMeterRow>;

    reload(in_ability_mode: boolean): void {
        this.newRows = new Map();
        this.instanceDataService.melee_damage
            .pipe(take(1))
            .subscribe(damage => {
                damage.forEach(event => this.feed_melee_damage(event, in_ability_mode));
                this.commit();
            });
        this.instanceDataService.spell_casts
            .pipe(take(1))
            .subscribe(spell_casts => {
                this.instanceDataService.spell_damage
                    .pipe(take(1))
                    .subscribe(damage => {
                        damage.forEach(event => this.feed_spell_damage(spell_casts, event, in_ability_mode));
                        this.commit();
                    });
            });
    }

    commit(): void {
        this.rows$.next(new Array<RaidMeterRow>(...this.newRows.values()));
    }

    private feed_melee_damage(event: Event, in_ability_mode: boolean): void {
        const damage = (event.event as any).MeleeDamage as Damage;
        const row_id = in_ability_mode ? 0 : get_unit_id(damage.victim);
        if (this.newRows.has(row_id)) {
            const row = this.newRows.get(row_id);
            row.amount += damage.damage;
        } else {
            this.newRows.set(row_id, {
                subject: in_ability_mode ? this.utilService.get_row_ability_subject_auto_attack() : this.utilService.get_row_unit_subject(damage.victim),
                amount: damage.damage
            });
        }
    }

    private feed_spell_damage(spell_casts: Array<Event>, event: Event, in_ability_mode: boolean): void {
        const damage = (event.event as any).SpellDamage.damage as Damage;
        const spell_cast_id = ((event.event as any).SpellDamage).spell_cast_id as number;
        const spell_cast_event = spell_casts.find(cast => cast.id === spell_cast_id);
        if (spell_cast_event === undefined)
            return;
        const spell_cast = (spell_cast_event.event as any).SpellCast as SpellCast;
        const row_id = in_ability_mode ? spell_cast.spell_id : get_unit_id(damage.victim);
        if (this.newRows.has(row_id)) {
            const row = this.newRows.get(row_id);
            row.amount += damage.damage;
        } else {
            this.newRows.set(row_id, {
                subject: in_ability_mode ? this.utilService.get_row_ability_subject(spell_cast.spell_id) : this.utilService.get_row_unit_subject(damage.victim),
                amount: damage.damage
            });
        }
    }

}
