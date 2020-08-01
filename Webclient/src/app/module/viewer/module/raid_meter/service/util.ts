import {Injectable, OnDestroy} from "@angular/core";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {RaidMeterSubject} from "../domain_value/raid_meter_subject";
import {UnitService} from "../../../service/unit";
import {EventType} from "../../../domain_value/event_type";
import {Observable, of, Subscription} from "rxjs";
import {InstanceViewerMeta} from "../../../domain_value/instance_viewer_meta";
import {InstanceDataService} from "../../../service/instance_data";
import {DataService} from "../../../../../service/data";
import {Localized} from "../../../../../domain_value/localized";
import {BasicSpell} from "../../../../../domain_value/data/basic_spell";
import {concatMap, map} from "rxjs/operators";
import {first_matching_primary_school} from "../../../../../stdlib/spell";

@Injectable({
    providedIn: "root",
})
export class UtilService implements OnDestroy {

    private subscription: Subscription;
    private current_meta: InstanceViewerMeta;

    constructor(
        private instanceDataService: InstanceDataService,
        private dataService: DataService,
        private unitService: UnitService
    ) {
        this.subscription = this.instanceDataService.meta.subscribe(meta => this.current_meta = meta);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
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
        const id = 0;
        const name = of("Auto Attack");
        const color_class = of("spell_school_bg_0");
        const icon = of("/assets/wow_icon/inv_sword_04.jpg");
        return {id, name, color_class, icon};
    }

    get_row_ability_subject(spell_id: number): RaidMeterSubject {
        const basic_spell = this.get_localized_basic_spell(spell_id);
        const name = basic_spell.pipe(map(spell => spell?.localization));
        const color_class = basic_spell.pipe(map(spell => "spell_school_bg_" + first_matching_primary_school(spell?.base.school)));
        const icon = basic_spell.pipe(map(spell => "/assets/wow_icon/" + spell?.base.icon + ".jpg"));
        return {id: spell_id, name, color_class, icon};
    }

    private get_localized_basic_spell(spell_id: number): Observable<Localized<BasicSpell>> {
        if (!this.current_meta)
            return of(undefined);
        return this.dataService.get_server_by_id(this.current_meta.server_id)
            .pipe(concatMap(server => this.dataService.get_localized_basic_spell(server.expansion_id, spell_id)));
    }

}
