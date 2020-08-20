import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Event} from "../../../domain_value/event";
import {get_heal, get_spell_cause, get_spell_damage} from "../../../extractor/events";

export class RaidEventLogKnecht {
    private static readonly PAGE_SIZE: number = 25;

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    private static slice_and_dice(container: Array<Event>, offset: number, up_to_timestamp: number): Array<Event> {
        const result = up_to_timestamp === 0 ? container : container.filter(event => event.timestamp <= up_to_timestamp);
        if (offset === 0)
            return result.slice(-RaidEventLogKnecht.PAGE_SIZE);
        return result.slice(-(RaidEventLogKnecht.PAGE_SIZE + offset), -offset);
    }

    async get_melee_damage_entries(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<Event>> {
        return RaidEventLogKnecht.slice_and_dice(this.data_filter.get_melee_damage(inverse), offset, up_to_timestamp);
    }

    async get_spell_damage_entries(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<[Event, [boolean, Event]]>> {
        const event_map = this.data_filter.get_event_map();
        return RaidEventLogKnecht.slice_and_dice(this.data_filter.get_spell_damage(inverse), offset, up_to_timestamp)
            .map(spell_event => [spell_event, get_spell_cause(get_spell_damage(spell_event).spell_cause_id, event_map)]);
    }

    async get_heal_entries(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<[Event, [boolean, Event]]>> {
        const event_map = this.data_filter.get_event_map();
        return RaidEventLogKnecht.slice_and_dice(this.data_filter.get_heal(inverse), offset, up_to_timestamp)
            .map(spell_event => [spell_event, get_spell_cause(get_heal(spell_event).spell_cause_id, event_map)]);
    }

    async get_death_entries(inverse: boolean, offset: number = 0, up_to_timestamp: number = 0): Promise<Array<Event>> {
        return RaidEventLogKnecht.slice_and_dice(this.data_filter.get_deaths(inverse), offset, up_to_timestamp);
    }
}
