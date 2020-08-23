import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {te_dispel} from "../../../extractor/targets";
import {se_dispel} from "../../../extractor/sources";
import {commit_dispel} from "../stdlib/dispel";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";

export class RaidMeterDispel {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, [Unit, Array<UnAuraOverviewRow>]]>> {
        if (inverse)
            return commit_dispel(this.data_filter.get_dispels(true), this.data_filter.get_event_map(), te_dispel(this.data_filter.get_event_map()));
        return commit_dispel(this.data_filter.get_dispels(false), this.data_filter.get_event_map(), se_dispel(this.data_filter.get_event_map()));
    }
}
