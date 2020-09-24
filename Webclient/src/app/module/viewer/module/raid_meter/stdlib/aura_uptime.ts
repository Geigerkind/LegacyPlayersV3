import {Event} from "../../../domain_value/event";
import {get_unit_id, get_unit_owner, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {se_aura_application} from "../../../extractor/sources";
import {te_aura_application} from "../../../extractor/targets";


export function commit_aura_uptime(aura_applications: Array<Event>, current_segment_intervals: Array<[number, number]>):
    Array<[number, [Unit, Array<[number, Array<[number | undefined, number | undefined, Unit | undefined, Unit | undefined]>]>]]> {
    if (current_segment_intervals.length === 0)
        return [];

    const newData = new Map<number, [Unit, Map<number, Array<[number | undefined, number | undefined, Unit | undefined, Unit | undefined]>>]>();

    const grouping = group_by(aura_applications, (event) => get_unit_id(se_aura_application(event), false));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!newData.has(subject_id))
            newData.set(subject_id, [se_aura_application(grouping[unit_id][0]), new Map()]);

        const abilities_data = newData.get(subject_id)[1];
        grouping[subject_id].forEach(event => {
            const spell_id = event[4];
            if (!abilities_data.has(spell_id)) {
                abilities_data.set(spell_id, []);
            }
            const ability_intervals = abilities_data.get(spell_id);
            let current_interval;
            if (ability_intervals.length > 0) current_interval = ability_intervals[ability_intervals.length - 1];
            else {
                current_interval = [undefined, undefined, undefined, undefined];
                ability_intervals.push(current_interval);
            }
            if (event[5] > 0) {
                // If we get an non 0 event twice, it is probably a refresh
                if (current_interval[0] === undefined) {
                    current_interval[0] = event[1];
                    current_interval[2] = te_aura_application(event);
                } else if (current_interval[1] !== undefined) {
                    ability_intervals.push([event[1], undefined, te_aura_application(event), undefined]);
                } else if (event[1] > current_interval[0]) {
                    current_interval[1] = event[1];
                    current_interval[3] = te_aura_application(event);
                    ability_intervals.push([event[1], undefined, te_aura_application(event), undefined]);
                }
            } else {
                if (current_interval[1] === undefined) {
                    current_interval[1] = event[1];
                    current_interval[3] = te_aura_application(event);
                } else if (current_interval[0] !== undefined
                    && ability_intervals.find(i_interval => i_interval[0] === current_interval[0] && i_interval[1] === event[1]) === undefined) {
                    ability_intervals.push([current_interval[0], event[1], current_interval[2], te_aura_application(event)]);
                }
            }
        });
    }

    const segment_end = current_segment_intervals.reduce((acc, [start, end]) => Math.max(acc, end), 0);
    return [...newData.entries()].map(([unit_id, [unit, abilities]]) => {
        const result = [];
        for (const [spell_id, intervals] of abilities) {
            const res_intervals = [];
            for (const [start, end, start_id, end_id] of intervals) {
                /*
                 * Should not be possible?
                if (start === undefined && end === undefined) {
                    res_intervals.push([segment_start, segment_end]);
                    break;
                }
                 */

                const int_end = end === undefined ? segment_end : end;
                for (const [ci_start, ci_end] of current_segment_intervals) {
                    if (ci_start > end)
                        break;
                    if (start === undefined || start <= ci_start) {
                        if (ci_end <= int_end) res_intervals.push([ci_start, ci_end, start_id, end_id]);
                        else res_intervals.push([ci_start, int_end, start_id, end_id]);
                    } else if (start < ci_end) {
                        if (ci_end <= int_end) res_intervals.push([start, ci_end, start_id, end_id]);
                        else res_intervals.push([start, int_end, start_id, end_id]);
                    }
                }
            }
            if (res_intervals.length > 0)
                result.push([spell_id, res_intervals]);
        }
        return [unit_id, [unit, result]];
    });
}

export function flatten_aura_uptime_to_spell_map(data: Array<[number, Array<[number, Array<[number, number]>]>]>): Array<[number, Array<[number, number]>]> {
    const result = [];
    const ability_intervals = new Map<number, Array<[number, number]>>();
    for (const [unit_id, abilities] of data) {
        // @ts-ignore
        for (const [spell_id, intervals] of abilities) {
            // @ts-ignore
            if (!ability_intervals.has(spell_id)) ability_intervals.set(spell_id, intervals);
            else { // @ts-ignore
                ability_intervals.set(spell_id, [...ability_intervals.get(spell_id), ...intervals]);
            }
        }
    }
    // @ts-ignore
    for (let [spell_id, intervals] of ability_intervals) {
        intervals = intervals.sort((left, right) => left[1] - right[1]);
        let previous_intervals = [intervals[0]];
        for (let i = 1; i < intervals.length; ++i) {
            let current_interval = intervals[i];
            let new_intervals = [];
            for (let m = previous_intervals.length - 1; m >= 0; --m) {
                const previous_interval = previous_intervals[m];
                if (previous_interval[1] < current_interval[0]) {
                    new_intervals = previous_intervals;
                    new_intervals.push(current_interval);
                    break;
                } else {
                    const new_interval: [number, number] = [previous_interval[0], current_interval[1]];
                    new_intervals.push(new_interval);
                    current_interval = new_interval;
                }
            }
            previous_intervals = new_intervals.sort((left, right) => left[1] - right[1]);
        }
        result.push([spell_id, previous_intervals]);
    }
    return result;
}

export function flatten_aura_uptime_to_subject_map(data: Array<[number, Array<[number, Array<[number, number]>]>]>): Array<[number, Array<[number, number]>]> {
    const result = [];
    const unit_intervals = new Map<number, Array<[number, number]>>();
    for (const [unit_id, abilities] of data) {
        // @ts-ignore
        for (const [spell_id, intervals] of abilities) {
            // @ts-ignore
            if (!unit_intervals.has(unit_id)) unit_intervals.set(unit_id, intervals);
            else { // @ts-ignore
                unit_intervals.set(unit_id, [...unit_intervals.get(unit_id), ...intervals]);
            }
        }
    }
    // @ts-ignore
    for (let [unit_id, intervals] of unit_intervals) {
        intervals = intervals.sort((left, right) => left[1] - right[1]);
        let previous_intervals = [intervals[0]];
        for (let i = 1; i < intervals.length; ++i) {
            let current_interval = intervals[i];
            let new_intervals = [];
            for (let m = previous_intervals.length - 1; m >= 0; --m) {
                const previous_interval = previous_intervals[m];
                if (previous_interval[1] < current_interval[0]) {
                    new_intervals = previous_intervals;
                    new_intervals.push(current_interval);
                    break;
                } else {
                    const new_interval: [number, number] = [previous_interval[0], current_interval[1]];
                    new_intervals.push(new_interval);
                    current_interval = new_interval;
                }
            }
            previous_intervals = new_intervals.sort((left, right) => left[1] - right[1]);
        }
        result.push([unit_id, previous_intervals]);
    }
    return result;
}
