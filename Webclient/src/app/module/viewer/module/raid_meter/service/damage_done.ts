import {Injectable} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {RaidMeterRow} from "../domain_value/raid_meter_row";
import {BehaviorSubject, Observable} from "rxjs";
import {map, take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {get_unit_id} from "../../../domain_value/unit";
import {EventType} from "../../../domain_value/event_type";
import {MeleeDamage} from "../../../domain_value/melee_damage";
import {SpellDamage} from "../../../domain_value/spell_damage";
import {Damage} from "../../../domain_value/damage";
import {UnitService} from "../../../service/unit";
import {InstanceViewerMeta} from "../../../domain_value/instance_viewer_meta";

@Injectable({
    providedIn: "root",
})
export class DamageDoneService {

    constructor(
        private instanceDataService: InstanceDataService,
        private unitService: UnitService
    ) {
    }

    get rows(): Observable<Array<RaidMeterRow>> {
        return this.rows$.asObservable()
            .pipe(map(rows => rows.sort((left, right) => right.amount - left.amount)));
    }

    private rows$: BehaviorSubject<Array<RaidMeterRow>> = new BehaviorSubject([]);
    private newRows: Map<number, RaidMeterRow>;

    private static extract_damage_from_melee_damage(damage: EventType): number {
        return ((damage as any).MeleeDamage as MeleeDamage).damage;
    }

    private static extract_damage_from_spell_damage(damage: EventType): number {
        return (((damage as any).SpellDamage as SpellDamage).damage as Damage).damage;
    }

    reload(): void {
        this.newRows = new Map();
        this.instanceDataService.meta
            .pipe(take(1))
            .subscribe(meta => {
            this.instanceDataService.melee_damage
                .pipe(take(1))
                .subscribe(damage => {
                    damage.forEach(event => this.feed_damage(meta, event, DamageDoneService.extract_damage_from_melee_damage));
                    this.commit();
                });
            this.instanceDataService.spell_damage
                .pipe(take(1))
                .subscribe(damage => {
                    damage.forEach(event => this.feed_damage(meta, event, DamageDoneService.extract_damage_from_spell_damage));
                    this.commit();
                });
        });
    }

    commit(): void {
        this.rows$.next(new Array<RaidMeterRow>(...this.newRows.values()));
    }

    private feed_damage(meta: InstanceViewerMeta, damage: Event, damage_extract_function: any): void {
        const unit_id = get_unit_id(damage.subject);
        if (this.newRows.has(unit_id)) {
            const row = this.newRows.get(unit_id);
            row.amount += damage_extract_function(damage.event);
        } else {
            this.newRows.set(unit_id, {
                subject: {
                    id: unit_id,
                    name: this.unitService.get_unit_name(damage.subject, meta.server_id),
                    color_class: this.unitService.get_unit_bg_color(damage.subject),
                    icon: this.unitService.get_unit_icon(damage.subject)
                },
                amount: damage_extract_function(damage.event)
            });
        }
    }

}
