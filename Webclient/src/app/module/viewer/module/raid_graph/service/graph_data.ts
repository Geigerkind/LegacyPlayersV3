import {Injectable, OnDestroy} from "@angular/core";
import {InstanceDataService} from "../../../service/instance_data";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {DataSet, is_event_data_set} from "../domain_value/data_set";
import {take} from "rxjs/operators";
import {Event} from "../../../domain_value/event";
import {get_heal, get_melee_damage, get_spell_damage, get_threat} from "../../../extractor/events";

@Injectable({
    providedIn: "root",
})
export class GraphDataService implements OnDestroy {

    private subscription: Subscription;
    private data_points$: BehaviorSubject<[Array<number>, Map<DataSet, [Array<number>, Array<number>]>]> = new BehaviorSubject([[], new Map()]);
    private temp_data_set: Map<DataSet, Map<number, number>> = new Map();

    constructor(
        private instanceDataService: InstanceDataService
    ) {
        this.subscription = this.instanceDataService.changed.subscribe(changed => {
            for (const data_set of [...this.data_points$.getValue()[1].keys()])
                this.add_data_set(data_set);
        });

    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    get data_points(): Observable<[Array<number>, Map<DataSet, [Array<number>, Array<number>]>]> {
        return this.data_points$.asObservable();
    }

    update(): void {
        this.data_points$.next(this.data_points$.getValue());
    }

    add_data_set(data_set: DataSet): void {
        this.temp_data_set.set(data_set, new Map());
        switch (data_set) {
            case DataSet.DamageDone:
            case DataSet.DamageTaken:
                this.instanceDataService.get_melee_damage(data_set === DataSet.DamageTaken)
                    .pipe(take(1))
                    .subscribe(damage => {
                        this.feed_melee_damage(data_set, [...damage.values()]);
                        this.commit_data_set(data_set);
                    });
                this.instanceDataService.get_spell_damage(data_set === DataSet.DamageTaken)
                    .pipe(take(1))
                    .subscribe(damage => {
                        this.feed_spell_damage(data_set, damage);
                        this.commit_data_set(data_set);
                    });
                break;
            case DataSet.TotalHealingDone:
            case DataSet.TotalHealingTaken:
            case DataSet.EffectiveHealingDone:
            case DataSet.EffectiveHealingTaken:
            case DataSet.OverhealingDone:
            case DataSet.OverhealingTaken:
                this.instanceDataService.get_heal([DataSet.TotalHealingTaken, DataSet.EffectiveHealingTaken, DataSet.OverhealingTaken].includes(data_set))
                    .pipe(take(1))
                    .subscribe(heal => {
                        this.feed_heal(data_set, heal);
                        this.commit_data_set(data_set);
                    });
                break;
            case DataSet.ThreatDone:
                this.instanceDataService.get_threat()
                    .pipe(take(1))
                    .subscribe(threat => {
                        this.feed_threat(data_set, threat);
                        this.commit_data_set(data_set);
                    });
                break;
            case DataSet.Deaths:
            case DataSet.Kills:
                this.instanceDataService.get_deaths(data_set === DataSet.Kills)
                    .pipe(take(1))
                    .subscribe(death => {
                        this.feed_death(data_set, death);
                        this.commit_data_set(data_set);
                    });
                break;
        }
    }

    commit_data_set(data_set: DataSet): void {
        const [old_total_x_axis, data_points] = this.data_points$.getValue();
        const x_axis = new Array<number>();
        const y_axis = [];
        for (const [x, y] of [...this.temp_data_set.get(data_set).entries()]
            .sort(([left_x, left_y], [right_x, right_y]) => left_x - right_x)) {
            x_axis.push(x);
            y_axis.push(y);
        }

        if (is_event_data_set(data_set)) {
            let max_value = [...data_points.values()]
                .map(([x, y]) => y)
                .reduce((acc, y) => Math.max(acc, ...y), 0);
            max_value *= 0.75;
            for (let i = 0; i < y_axis.length; ++i)
                y_axis[i] = max_value;
        }

        data_points.set(data_set, [x_axis, y_axis]);
        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    remove_data_set(data_set: DataSet): void {
        const [old_x_axis, data_points] = this.data_points$.getValue();
        data_points.delete(data_set);
        this.data_points$.next([this.compute_x_axis(data_points), data_points]);
    }

    private compute_x_axis(data_points: Map<DataSet, [Array<number>, Array<number>]>): Array<number> {
        let result = new Set<number>();
        for (const [timestamps, values] of data_points.values()) {
            result = new Set<number>([...result.values(), ...timestamps.values()]);
        }
        return [...result.values()].sort((left, right) => left - right);
    }

    private feed_melee_damage(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            const damage = get_melee_damage(event).damage;
            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + damage);
            else points.set(event.timestamp, damage);
        }
    }

    private feed_spell_damage(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            const damage = get_spell_damage(event).damage.damage;
            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + damage);
            else points.set(event.timestamp, damage);
        }
    }

    private feed_heal(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            const heal_event = get_heal(event);
            let healing;
            if ([DataSet.TotalHealingDone, DataSet.TotalHealingTaken].includes(data_set)) healing = heal_event.heal.total;
            else if ([DataSet.EffectiveHealingDone, DataSet.EffectiveHealingTaken].includes(data_set)) healing = heal_event.heal.effective;
            else healing = heal_event.heal.total - heal_event.heal.effective;
            if (![DataSet.OverhealingDone, DataSet.OverhealingTaken].includes(data_set) || healing > 0) {
                if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + healing);
                else points.set(event.timestamp, healing);
            }
        }
    }

    private feed_threat(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            const threat = get_threat(event).threat.amount;
            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + threat);
            else points.set(event.timestamp, threat);
        }
    }

    private feed_death(data_set: DataSet, events: Array<Event>): void {
        const points = this.temp_data_set.get(data_set);
        for (const event of events) {
            if (points.has(event.timestamp)) points.set(event.timestamp, points.get(event.timestamp) + 1);
            else points.set(event.timestamp, 1);
        }
    }
}
