import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {DataSet, is_event_data_set} from "../domain_value/data_set";
import {Event} from "../../../domain_value/event";
import {get_spell_components_total_amount} from "../../../domain_value/damage";
import {Unit} from "../../../domain_value/unit";
import {se_death} from "../../../extractor/sources";
import {te_death} from "../../../extractor/targets";

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

        /*
        // Normalize amount
        squash_time /= this.SQUASH_TIME_MULTIPLIER;
        const final_res = [];
        for (const [timestamp, amount] of data_points)
            final_res.push([timestamp, Math.round(amount / squash_time)]);
         */
        return data_points;
    }

    private static feed_points(events: Array<Event>, extract_amount: (Event) => number | Unit): Array<[number, number | Unit]> {
        const result = [];
        for (const event of events)
            result.push([event[1], extract_amount(event)]);
        return result;
    }

    async get_data_set(data_set: DataSet): Promise<Array<[number, number | Unit]>> {
        const data_points = this.extract_data_points(data_set);
        if (is_event_data_set(data_set))
            return data_points;
        return RaidGraphKnecht.squash(data_points as Array<[number, number]>);
    }

    private extract_data_points(data_set: DataSet): Array<[number, number | Unit]> {
        return (() => {
            switch (data_set) {
                case DataSet.DamageDone:
                case DataSet.DamageTaken:
                    return [...RaidGraphKnecht.feed_points(this.data_filter.get_melee_damage(data_set === DataSet.DamageTaken),
                        (event) => get_spell_components_total_amount(event[5])),
                        ...RaidGraphKnecht.feed_points(this.data_filter.get_spell_damage(data_set === DataSet.DamageTaken),
                            (event) => get_spell_components_total_amount(event[7]))];
                case DataSet.TotalHealingDone:
                case DataSet.TotalHealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.TotalHealingTaken), (event) => event[8]);
                case DataSet.EffectiveHealingDone:
                case DataSet.EffectiveHealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.EffectiveHealingTaken), (event) => event[9]);
                case DataSet.OverhealingDone:
                case DataSet.OverhealingTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_heal(data_set === DataSet.OverhealingTaken), (event) => event[8] - event[9]);
                case DataSet.ThreatDone:
                case DataSet.ThreatTaken:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_threat(data_set === DataSet.ThreatTaken), (event) => event[8]);
                case DataSet.Deaths:
                case DataSet.Kills:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_deaths(data_set === DataSet.Kills), (event) => {
                        if (data_set === DataSet.Kills) {
                            return te_death(event);
                        }
                        return se_death(event);
                    });
                case DataSet.DispelsDone:
                case DataSet.DispelsReceived:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_dispels(data_set === DataSet.DispelsReceived), (event) => 1);
                case DataSet.InterruptDone:
                case DataSet.InterruptReceived:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_interrupts(data_set === DataSet.InterruptReceived), (event) => 1);
                case DataSet.SpellStealDone:
                case DataSet.SpellStealReceived:
                    return RaidGraphKnecht.feed_points(this.data_filter.get_spell_steals(data_set === DataSet.SpellStealReceived), (event) => 1);
            }
            return [];
        })().sort((left, right) => left[0] - right[0]);
    }
}
