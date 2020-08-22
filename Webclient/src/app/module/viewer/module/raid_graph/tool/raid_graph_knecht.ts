import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {DataSet, is_event_data_set} from "../domain_value/data_set";
import {Event} from "../../../domain_value/event";
import {get_heal, get_melee_damage, get_spell_damage, get_threat} from "../../../extractor/events";
import {get_damage_components_total_damage} from "../../../domain_value/damage";

export class RaidGraphKnecht {
    private static readonly MAX_DATA_POINTS: number = 500;
    private static readonly DEFAULT_SQUASH_TIME_IN_MS: number = 100;
    private static readonly SQUASH_TIME_MULTIPLIER: number = 1.5;

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    static squash(data_points: Array<[number, number]>): Array<[number, number]> {
        if (data_points.length < this.MAX_DATA_POINTS)
            return data_points;

        let result = new Map();
        let squash_time = this.DEFAULT_SQUASH_TIME_IN_MS;
        while (data_points.length > this.MAX_DATA_POINTS) {
            result = new Map();
            const half_squash_time = Math.round(squash_time / 2);
            let current_first_timestamp = data_points[0][0];
            for (const [timestamp, amount] of data_points) {
                if (timestamp - current_first_timestamp > squash_time)
                    current_first_timestamp = timestamp;
                const key_ts = current_first_timestamp + half_squash_time;
                if (result.has(key_ts)) result.set(key_ts, result.get(key_ts) + amount);
                else result.set(current_first_timestamp + squash_time, amount);
            }
            squash_time *= this.SQUASH_TIME_MULTIPLIER;
            data_points = [...result.entries()];
        }
        return [...result.entries()];
    }

    private static feed_points(events: Array<Event>, extract_amount: (Event) => number): Array<[number, number]> {
        const result = [];
        for (const event of events)
            result.push([event.timestamp, extract_amount(event)]);
        return result;
    }

    async get_data_set(data_set: DataSet): Promise<Array<[number, number]>> {
        const data_points = this.extract_data_points(data_set);
        if (is_event_data_set(data_set))
            return data_points;
        return RaidGraphKnecht.squash(data_points);
    }

    private extract_data_points(data_set: DataSet): Array<[number, number]> {
        return (() => {
            switch (data_set) {
                case DataSet.DamageDone:
                case DataSet.DamageTaken:
                    return [...RaidGraphKnecht.feed_points(this.data_filter.get_melee_damage(data_set === DataSet.DamageTaken), (event) => get_damage_components_total_damage(get_melee_damage(event).damage_components)),
                        ...RaidGraphKnecht.feed_points(this.data_filter.get_spell_damage(data_set === DataSet.DamageTaken), (event) => get_damage_components_total_damage(get_spell_damage(event).damage.damage_components))];
                case DataSet.TotalHealingDone:
                case DataSet.TotalHealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.TotalHealingTaken), (event) => get_heal(event).heal.total);
                case DataSet.EffectiveHealingDone:
                case DataSet.EffectiveHealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.EffectiveHealingTaken), (event) => get_heal(event).heal.effective);
                case DataSet.OverhealingDone:
                case DataSet.OverhealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.OverhealingTaken), (event) => get_heal(event).heal.total - get_heal(event).heal.effective);
                case DataSet.ThreatDone:
                case DataSet.ThreatTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_threat(data_set === DataSet.ThreatTaken), (event) => get_threat(event).threat.amount);
                case DataSet.Deaths:
                case DataSet.Kills:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_deaths(data_set === DataSet.Kills), (event) => 1);
                case DataSet.DispelsDone:
                case DataSet.DispelsReceived:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_dispels(data_set === DataSet.DispelsReceived), (event) => 1);
            }
            return [];
        })().sort((left, right) => left[0] - right[0]);
    }
}
