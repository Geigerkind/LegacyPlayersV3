import {Injectable} from "@angular/core";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {UnitService} from "../../../service/unit";
import {of} from "rxjs";
import {map} from "rxjs/operators";
import {first_matching_primary_school} from "../../../../../stdlib/spell";
import {SpellService} from "../../../service/spell";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL} from "../../../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class UtilService {

    constructor(
        private unitService: UnitService,
        private spellService: SpellService
    ) {
    }

    get_row_unit_subject(unit: Unit): RaidMeterSubject {
        const unit_id = get_unit_id(unit);
        return {
            id: unit_id,
            name: this.unitService.get_unit_name(unit),
            color_class: this.unitService.get_unit_bg_color(unit),
            icon: this.unitService.get_unit_icon(unit)
        };
    }

    get_row_ability_subject_auto_attack(): RaidMeterSubject {
        const id = CONST_AUTO_ATTACK_ID;
        const name = of(CONST_AUTO_ATTACK_LABEL);
        const color_class = of("spell_school_bg_0");
        const icon = of("/assets/wow_icon/inv_sword_04.jpg");
        return {id, name, color_class, icon};
    }

    get_row_ability_subject(spell_id: number): RaidMeterSubject {
        if (spell_id === 0)
            return this.get_row_ability_subject_auto_attack();

        const basic_spell = this.spellService.get_localized_basic_spell(spell_id);
        const name = basic_spell.pipe(map(spell => spell?.localization));
        const color_class = basic_spell.pipe(map(spell => "spell_school_bg_" + first_matching_primary_school(spell?.base.school)));
        const icon = basic_spell.pipe(map(spell => "/assets/wow_icon/" + spell?.base.icon + ".jpg"));
        return {id: spell_id, name, color_class, icon};
    }

}
