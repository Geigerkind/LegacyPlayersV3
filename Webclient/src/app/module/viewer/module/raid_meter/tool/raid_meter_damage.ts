import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {commit_damage} from "../stdlib/damage";
import {te_melee_damage, te_spell_damage} from "../../../extractor/targets";
import {Unit} from "../../../domain_value/unit";
import {se_aura_app_or_own, se_identity} from "../../../extractor/sources";
import {ce_spell_damage} from "../../../extractor/causes";

export class RaidMeterDamage {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, [Unit, Array<[number, number]>]]>> {
        if (inverse)
            return commit_damage(this.data_filter.get_melee_damage(true), this.data_filter.get_spell_damage(true),
                this.data_filter.get_event_map(), te_melee_damage, te_spell_damage);
        return commit_damage(this.data_filter.get_melee_damage(false), this.data_filter.get_spell_damage(false),
            this.data_filter.get_event_map(), se_identity, se_aura_app_or_own(ce_spell_damage, this.data_filter.get_event_map()));
    }
}
