import {Injectable} from "@angular/core";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {RaidMeterSubject} from "../domain_value/raid_meter_subject";
import {UnitService} from "../../../service/unit";

@Injectable({
    providedIn: "root",
})
export class UtilService {

    constructor(
        private unitService: UnitService
    ) {
    }

    get_row_subject(unit: Unit, server_id: number): RaidMeterSubject {
        const unit_id = get_unit_id(unit);
        return {
            id: unit_id,
            name: this.unitService.get_unit_name(unit, server_id),
            color_class: this.unitService.get_unit_bg_color(unit),
            icon: this.unitService.get_unit_icon(unit)
        };
    }

}
