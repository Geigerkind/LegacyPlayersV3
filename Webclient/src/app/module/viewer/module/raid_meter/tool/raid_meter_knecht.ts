import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {RaidMeterHeal} from "./raid_meter_heal";
import {RaidMeterDamage} from "./raid_meter_damage";
import {RaidMeterThreat} from "./raid_meter_threat";
import {RaidMeterDeath} from "./raid_meter_death";
import {RaidMeterDispel} from "./raid_meter_dispel";

export class RaidMeterKnecht {

    public damage: RaidMeterDamage;
    public heal: RaidMeterHeal;
    public threat: RaidMeterThreat;
    public death: RaidMeterDeath;
    public dispel: RaidMeterDispel;

    constructor(
        private data_filter: InstanceDataFilter
    ) {
        this.damage = new RaidMeterDamage(data_filter);
        this.heal = new RaidMeterHeal(data_filter);
        this.threat = new RaidMeterThreat(data_filter);
        this.death = new RaidMeterDeath(data_filter);
        this.dispel = new RaidMeterDispel(data_filter);
    }
}
