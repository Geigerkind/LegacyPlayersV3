import {InstanceDataService} from "../service/instance_data";
import {Unit} from "../domain_value/unit";
import {get_all_schools, School} from "../domain_value/school";

export const ABSORBING_SPELL_IDS: Map<number, [Array<School>, Array<School>]> = new Map([
    [25218, [[School.Holy], get_all_schools()]]
]);

export async function get_absorb_data_points(current_mode: boolean, instance_data_service: InstanceDataService): Promise<Array<[number, [Unit, Array<[number, number, number]>]]>> {
    const aura_uptime = await instance_data_service.knecht_spell.meter_aura_uptime(current_mode);
    const absorb_melee = await instance_data_service.knecht_melee.meter_absorbed_damage();
    const absorb_spell = await instance_data_service.knecht_spell.meter_absorbed_damage();

    const result = new Map();

    const aura_uptime_map = new Map(aura_uptime);
    for (const data_set of [absorb_melee, absorb_spell]) {
        for (const [subject_id, [subject, absorbs]] of data_set) {
            if (!aura_uptime_map.has(subject_id))
                continue;
            const subject_intervals = new Map(aura_uptime_map.get(subject_id)[1]);
            for (const [spell_id, timestamp, amount] of absorbs) {
                for (const absorb_spell_id of ABSORBING_SPELL_IDS.keys()) {
                    if (!subject_intervals.has(absorb_spell_id))
                        continue;
                    const uptime_intervals = subject_intervals.get(absorb_spell_id);
                    for (const [start, end] of uptime_intervals) {
                        if ((start === undefined || start <= timestamp) && (end === undefined || end >= timestamp)) {
                            if (!result.has(subject_id))
                                result.set(subject_id, [subject, []]);
                            result.get(subject_id)[1].push([absorb_spell_id, timestamp, amount]);
                        }
                    }
                }
            }
        }
    }

    // @ts-ignore
    return [...result.entries()];
}
