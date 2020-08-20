import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {te_heal} from "../../../extractor/targets";
import {se_aura_app_or_own} from "../../../extractor/sources";
import {ce_heal} from "../../../extractor/causes";
import {commit_heal} from "../stdlib/heal";
import {HealMode} from "../../../domain_value/heal_mode";

export class RaidMeterHeal {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(heal_mode: HealMode, inverse: boolean): Promise<Array<[number, [Unit, Array<[number, number]>]]>> {
        if (inverse)
            return commit_heal(heal_mode, this.data_filter.get_heal(true), this.data_filter.get_event_map(), te_heal);
        return commit_heal(heal_mode, this.data_filter.get_heal(false), this.data_filter.get_event_map(), se_aura_app_or_own(ce_heal, this.data_filter.get_event_map()));
    }
}
