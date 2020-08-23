import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {te_spell_steal} from "../../../extractor/targets";
import {se_spell_steal} from "../../../extractor/sources";
import {commit_spell_steal} from "../stdlib/spell_steal";

export class RaidMeterSpellSteal {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, [Unit, Array<UnAuraOverviewRow>]]>> {
        if (inverse)
            return commit_spell_steal(this.data_filter.get_spell_steals(true), this.data_filter.get_event_map(), te_spell_steal(this.data_filter.get_event_map()));
        return commit_spell_steal(this.data_filter.get_spell_steals(false), this.data_filter.get_event_map(), se_spell_steal(this.data_filter.get_event_map()));
    }
}
