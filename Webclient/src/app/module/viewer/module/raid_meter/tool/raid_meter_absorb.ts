import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {te_melee_damage, te_spell_damage} from "../../../extractor/targets";
import {commit_absorb_damages} from "../stdlib/absorb";

export class RaidMeterAbsorb {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(): Promise<Array<[number, [Unit, Array<[number, number, number]>]]>> {
        return commit_absorb_damages(this.data_filter.get_melee_damage(true), this.data_filter.get_spell_damage(true),
            this.data_filter.get_event_map(), te_melee_damage, te_spell_damage);
    }
}
