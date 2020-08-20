import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {commit_threat_detail} from "../stdlib/threat_detail";

export class RaidDetailThreat {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, Array<[HitType, DetailRow]>]>> {
        if (inverse)
            return commit_threat_detail(this.data_filter.get_threat(true), this.data_filter.get_event_map());
        return commit_threat_detail(this.data_filter.get_threat(false), this.data_filter.get_event_map());
    }
}
