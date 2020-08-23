import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {commit_interrupt} from "../stdlib/interrupt";
import {se_identity, se_interrupt} from "../../../extractor/sources";

export class RaidMeterInterrupt {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, [Unit, Array<UnAuraOverviewRow>]]>> {
        if (inverse)
            return commit_interrupt(this.data_filter.get_interrupts(true), this.data_filter.get_event_map(), se_identity);
        return commit_interrupt(this.data_filter.get_interrupts(false), this.data_filter.get_event_map(), se_interrupt(this.data_filter.get_event_map()));
    }
}
