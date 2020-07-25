import {BehaviorSubject} from "rxjs";

function get_map_from_nested_array<K1, K2, V>(array: Array<[K1, Array<[K2, V]>]>): Map<K1, Map<K2, V>> {
    const result = new Map<K1, Map<K2, V>>();
    for (const [k1, inner_map] of array) {
        for (const [k2, v] of inner_map)
        {
            if (!result.has(k1))
                result.set(k1, new Map());
            if (!!v) result.get(k1).set(k2, v);
        }
    }
    return result;
}

function get_behavior_subject_map_from_nested_array<K1, K2, V>(array: Array<[K1, Array<[K2, V]>]>): Map<K1, Map<K2, BehaviorSubject<V>>> {
    const result = new Map<K1, Map<K2, BehaviorSubject<V>>>();
    for (const [k1, inner_map] of array) {
        for (const [k2, v] of inner_map)
        {
            if (!result.has(k1))
                result.set(k1, new Map());
            if (!!v) result.get(k1).set(k2, new BehaviorSubject<V>(v));
        }
    }
    return result;
}

function create_array_from_nested_map<K1, K2, V>(map: Map<K1, Map<K2, V>>): Array<[K1, Array<[K2, V]>]> {
    const result = [];
    // @ts-ignore
    for (const [k1, inner_map] of [...map.entries()])
        result.push([k1, [...inner_map.entries()]]);
    return result;
}

function create_array_from_nested_behavior_subject_map<K1, K2, V>(map: Map<K1, Map<K2, BehaviorSubject<V>>>): Array<[K1, Array<[K2, V]>]> {
    const result = [];
    // @ts-ignore
    for (const [k1, inner_map] of [...map.entries()]) {
        const inner_array = [];
        for (const [k2, v] of [...inner_map.entries()]) {
            if (!!v) inner_array.push([k2, v.getValue()]);
        }
        result.push([k1, inner_array]);
    }
    return result;
}

export {get_map_from_nested_array, create_array_from_nested_map, get_behavior_subject_map_from_nested_array, create_array_from_nested_behavior_subject_map};
