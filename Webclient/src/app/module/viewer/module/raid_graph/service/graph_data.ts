import {Injectable} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable} from "rxjs";
import {DataSet} from "../domain_value/data_set";
import {take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {MeleeDamage} from "../../../domain_value/melee_damage";
import {SpellDamage} from "../../../domain_value/spell_damage";

@Injectable({
    providedIn: "root",
})
export class GraphDataService {

    private data_points$: BehaviorSubject<[Array<number>, Map<DataSet, [Set<number>, Array<number>]>]> = new BehaviorSubject([[], new Map()]);
    private temp_data_set: Map<DataSet, Map<number, number>> = new Map();

    constructor(
        private instanceDataService: InstanceDataService
    ) {
        this.instanceDataService.changed.subscribe(changed => {
            for (const data_set of [...this.data_points$.getValue()[1].keys()])
                this.add_data_set(data_set);
        });

    }

    get data_points(): Observable<[Array<number>, Map<DataSet, [Set<number>, Array<number>]>]> {
        return this.data_points$.asObservable();
    }

    add_data_set(data_set: DataSet): void {
        this.temp_data_set.set(data_set, new Map());
        switch (data_set) {
            case DataSet.DamageDone:
            case DataSet.DamageTaken:
                this.instanceDataService.melee_damage
                    .pipe(take(1))
                    .subscribe(damage => {
                        this.feed_melee_damage(data_set, damage);
                        this.commit_data_set(data_set);
                    });
                this.instanceDataService.spell_damage
                    .pipe(take(1))
                    .subscribe(damage => {
                        this.feed_spell_damage(data_set, damage);
                        this.commit_data_set(data_set);
                    });
                break;
        }
    }

    commit_data_set(data_set: DataSet): void {
        const [old_total_x_axis, data_points] = this.data_points$.getValue();
        const x_axis = new Set<number>();
        const y_axis = [];
        for (const [x, y] of [...this.temp_data_set.get(data_set).entries()]
            .sort(([left_x, left_y], [right_x, right_y]) => left_x - right_x)) {
            x_axis.add(x);
            y_axis.push(y);
        }
        data_points.set(data_set, [x_axis, y_axis]);
        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    remove_data_set(data_set: DataSet): void {
        const [old_x_axis, data_points] = this.data_points$.getValue();
        data_points.delete(data_set);
        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    private compute_x_axis(data_points: Map<DataSet, [Set<number>, Array<number>]>): Array<number> {
        let result = new Set<number>();
        for (const [timestamps, values] of data_points.values())
            result = new Set<number>([...result.values(), ...timestamps.values()]);
        return [...result.values()].sort((left, right) => left - right);
    }

    private feed_melee_damage(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            let damage = ((event.event as any).MeleeDamage as MeleeDamage).damage;
            if (damage > 10000) damage = 0; // TODO: Remove later

            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + damage);
            else points.set(event.timestamp, damage);
        }
    }

    private feed_spell_damage(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            const damage = ((event.event as any).SpellDamage as SpellDamage).damage.damage;
            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + damage);
            else points.set(event.timestamp, damage);
        }
    }

}
