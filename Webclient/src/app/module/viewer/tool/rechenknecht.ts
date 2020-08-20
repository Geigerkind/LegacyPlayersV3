import {InstanceDataFilter} from "./instance_data_filter";
import {Unit} from "../domain_value/unit";
import {RaidMeterKnecht} from "../module/raid_meter/tool/raid_meter_knecht";
import {RaidGraphKnecht} from "../module/raid_graph/tool/raid_graph_knecht";
import {DataSet} from "../module/raid_graph/domain_value/data_set";
import {HealMode} from "../domain_value/heal_mode";
import {DeathOverviewRow} from "../module/raid_meter/module/deaths_overview/domain_value/death_overview_row";
import {UnAuraOverviewRow} from "../module/raid_meter/module/un_aura_overview/domain_value/un_aura_overview_row";
import {RaidDetailKnecht} from "../module/raid_detail_table/tool/raid_detail_knecht";
import {HitType} from "../domain_value/hit_type";
import {DetailRow} from "../module/raid_detail_table/domain_value/detail_row";
import {RaidEventLogKnecht} from "../module/raid_event_log/tool/raid_event_log_knecht";
import {Event} from "../domain_value/event";

export class Rechenknecht {
    constructor(
        private data_filter: InstanceDataFilter,
        private raid_meter_knecht: RaidMeterKnecht,
        private raid_graph_knecht: RaidGraphKnecht,
        private raid_detail_knecht: RaidDetailKnecht,
        private raid_event_log_knecht: RaidEventLogKnecht
    ) {
    }

    /*
     * RAID EVENT LOG
     */
    async event_log_melee_damage(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<Event>> {
        return this.raid_event_log_knecht.get_melee_damage_entries(inverse, offset, up_to_timestamp);
    }

    async event_log_spell_damage(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<[Event, [boolean, Event]]>> {
        return this.raid_event_log_knecht.get_spell_damage_entries(inverse, offset, up_to_timestamp);
    }

    async event_log_heal(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<[Event, [boolean, Event]]>> {
        return this.raid_event_log_knecht.get_heal_entries(inverse, offset, up_to_timestamp);
    }

    async event_log_deaths(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<Event>> {
        return this.raid_event_log_knecht.get_death_entries(inverse, offset, up_to_timestamp);
    }

    /*
     * RAID DETAIL
     */
    async detail_damage(inverse: boolean): Promise<Array<[number, Array<[HitType, DetailRow]>]>> {
        return this.raid_detail_knecht.damage.calculate(inverse);
    }

    async detail_heal(heal_mode: HealMode, inverse: boolean): Promise<Array<[number, Array<[HitType, DetailRow]>]>> {
        return this.raid_detail_knecht.heal.calculate(heal_mode, inverse);
    }

    async detail_threat(inverse: boolean): Promise<Array<[number, Array<[HitType, DetailRow]>]>> {
        return this.raid_detail_knecht.threat.calculate(inverse);
    }

    /*
     * RAID GRAPH
     */
    async graph_data_set(data_set: DataSet): Promise<Array<[number, number]>> {
        return this.raid_graph_knecht.get_data_set(data_set);
    }

    /*
     * RAID METER
     */
    async meter_damage(inverse: boolean): Promise<Array<[number, [Unit, Array<[number, number]>]]>> {
        return this.raid_meter_knecht.damage.calculate(inverse);
    }

    async meter_heal(heal_mode: HealMode, inverse: boolean): Promise<Array<[number, [Unit, Array<[number, number]>]]>> {
        return this.raid_meter_knecht.heal.calculate(heal_mode, inverse);
    }

    async meter_threat(inverse: boolean): Promise<Array<[number, [Unit, Array<[number, number]>]]>> {
        return this.raid_meter_knecht.threat.calculate(inverse);
    }

    async meter_death(inverse: boolean): Promise<Array<[number, [Unit, Array<DeathOverviewRow>]]>> {
        return this.raid_meter_knecht.death.calculate(inverse);
    }

    async meter_dispel(inverse: boolean): Promise<Array<[number, [Unit, Array<UnAuraOverviewRow>]]>> {
        return this.raid_meter_knecht.dispel.calculate(inverse);
    }

    /*
     * DATA FILTER
     */
    async set_segment_intervals(intervals: Array<[number, number]>): Promise<void> {
        await this.data_filter.set_segment_intervals(intervals);
    }

    async set_source_filter(sources: Array<number>): Promise<void> {
        await this.data_filter.set_source_filter(sources);
    }

    async set_target_filter(targets: Array<number>): Promise<void> {
        await this.data_filter.set_target_filter(targets);
    }

    async set_ability_filter(abilities: Array<number>): Promise<void> {
        await this.data_filter.set_ability_filter(abilities);
    }

    async get_sources(): Promise<Array<Unit>> {
        return await this.data_filter.get_sources();
    }

    async get_targets(): Promise<Array<Unit>> {
        return await this.data_filter.get_targets();
    }

    async get_abilities(): Promise<Array<number>> {
        return await this.data_filter.get_abilities();
    }

    async get_loot(): Promise<Array<Event>> {
        return this.data_filter.get_loot();
    }
}

