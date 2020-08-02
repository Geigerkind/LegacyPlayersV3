import {Injectable} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {RaidMeterRow} from "../domain_value/raid_meter_row";
import {BehaviorSubject, Observable} from "rxjs";
import {map, take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {get_unit_id} from "../../../domain_value/unit";
import {UtilService} from "./util";
import {Damage} from "../../../domain_value/damage";
import {SpellCast} from "../../../domain_value/spell_cast";
import {group_by} from "../../../../../stdlib/group_by";

@Injectable({
    providedIn: "root",
})
export class DamageDoneService {

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    get rows(): Observable<Array<RaidMeterRow>> {
        return this.rows$.asObservable()
            .pipe(map(rows => rows.sort((left, right) => right.amount - left.amount)));
    }

    get preview(): Observable<Map<number, Array<RaidMeterRow>>> {
        return this.preview$.asObservable();
    }

    private rows$: BehaviorSubject<Array<RaidMeterRow>> = new BehaviorSubject([]);
    private preview$: BehaviorSubject<Map<number, Array<RaidMeterRow>>> = new BehaviorSubject(new Map());
    private newRows: Map<number, RaidMeterRow>;
    private newPreview: Map<number, Map<number, RaidMeterRow>>;

    reload(in_ability_mode: boolean): void {
        this.newRows = new Map();
        this.load_events(in_ability_mode, (result_map) => {
            this.newRows = this.merge_maps(result_map, this.newRows);
            this.rows$.next(new Array<RaidMeterRow>(...this.newRows.values()));
        }, false);
    }

    reload_preview(): void {
        this.newPreview = new Map();
        this.load_events(true, (result_map, subject_id) => {
            if (this.newPreview.has(subject_id)) {
                this.newPreview.set(subject_id, this.merge_maps(result_map, this.newPreview.get(subject_id)));
            } else {
                this.newPreview.set(subject_id, result_map);
            }

            const result = new Map<number, Array<RaidMeterRow>>();
            for (const [key, inner_map] of this.newPreview.entries())
                result.set(Number(key), [...inner_map.values()]);
            this.preview$.next(result);
        }, true);
    }

    private load_events(in_ability_mode: boolean, commit: (map, subject_id) => void, preview: boolean): void {
        this.instanceDataService.melee_damage
            .pipe(take(1))
            .subscribe(damage => {
                if (preview) {
                    const grouping = group_by(damage, (event) => get_unit_id(event.subject));
                    for (const subject_id in grouping) {
                        const new_rows = new Map();
                        grouping[subject_id].forEach(event => this.feed_melee_damage(new_rows, event, in_ability_mode));
                        commit(new_rows, subject_id);
                    }
                } else {
                    const new_rows = new Map();
                    damage.forEach(event => this.feed_melee_damage(new_rows, event, in_ability_mode));
                    commit(new_rows, undefined);
                }
            });
        this.instanceDataService.spell_casts
            .pipe(take(1))
            .subscribe(spell_casts => {
                this.instanceDataService.spell_damage
                    .pipe(take(1))
                    .subscribe(damage => {
                        if (preview) {
                            const grouping = group_by(damage, (event) => get_unit_id(event.subject));
                            for (const subject_id in grouping) {
                                const new_rows = new Map();
                                grouping[subject_id].forEach(event => this.feed_spell_damage(new_rows, spell_casts, event, in_ability_mode));
                                commit(new_rows, subject_id);
                            }
                        } else {
                            const new_rows = new Map();
                            damage.forEach(event => this.feed_spell_damage(new_rows, spell_casts, event, in_ability_mode));
                            commit(new_rows, undefined);
                        }
                    });
            });
    }

    private merge_maps(map1: Map<number, RaidMeterRow>, map2: Map<number, RaidMeterRow>): Map<number, RaidMeterRow> {
        const result = map2;
        for (const [subject_id, row] of map1.entries()) {
            if (result.has(subject_id)) {
                const inner = result.get(subject_id);
                inner.amount += row.amount;
            } else {
                result.set(subject_id, row);
            }
        }
        return result;
    }

    private feed_melee_damage(new_rows: Map<number, RaidMeterRow>, event: Event, in_ability_mode: boolean): void {
        const damage = (event.event as any).MeleeDamage as Damage;
        const row_id = in_ability_mode ? 0 : get_unit_id(event.subject);

        if (new_rows.has(row_id)) {
            const row = new_rows.get(row_id);
            row.amount += damage.damage;
        } else {
            new_rows.set(row_id, {
                subject: in_ability_mode ? this.utilService.get_row_ability_subject_auto_attack() : this.utilService.get_row_unit_subject(event.subject),
                amount: damage.damage
            });
        }
    }

    private feed_spell_damage(new_rows: Map<number, RaidMeterRow>, spell_casts: Array<Event>, event: Event, in_ability_mode: boolean): void {
        const damage = (event.event as any).SpellDamage.damage as Damage;
        const spell_cast_id = ((event.event as any).SpellDamage).spell_cast_id as number;
        const spell_cast_event = spell_casts.find(cast => cast.id === spell_cast_id);
        if (spell_cast_event === undefined)
            return;
        const spell_cast = (spell_cast_event.event as any).SpellCast as SpellCast;
        const row_id = in_ability_mode ? spell_cast.spell_id : get_unit_id(event.subject);

        if (new_rows.has(row_id)) {
            const row = new_rows.get(row_id);
            row.amount += damage.damage;
        } else {
            new_rows.set(row_id, {
                subject: in_ability_mode ? this.utilService.get_row_ability_subject(spell_cast.spell_id) : this.utilService.get_row_unit_subject(event.subject),
                amount: damage.damage
            });
        }
    }

}
