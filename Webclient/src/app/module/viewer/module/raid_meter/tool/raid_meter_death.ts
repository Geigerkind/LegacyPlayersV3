import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {Unit} from "../../../domain_value/unit";
import {te_death, te_melee_damage, te_spell_damage} from "../../../extractor/targets";
import {se_identity} from "../../../extractor/sources";
import {commit_death} from "../stdlib/death";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";

export class RaidMeterDeath {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(inverse: boolean): Promise<Array<[number, [Unit, Array<DeathOverviewRow>]]>> {
        if (inverse)
            return commit_death(this.data_filter.get_deaths(true), this.data_filter.get_event_map(),
                this.data_filter.get_melee_damage(false), this.data_filter.get_spell_damage(false),
                te_melee_damage, te_spell_damage, te_death);
        return commit_death(this.data_filter.get_deaths(false), this.data_filter.get_event_map(),
            this.data_filter.get_melee_damage(true), this.data_filter.get_spell_damage(true),
            te_melee_damage, te_spell_damage, se_identity);
    }
}
