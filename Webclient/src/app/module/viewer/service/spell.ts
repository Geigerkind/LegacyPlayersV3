import {Injectable} from "@angular/core";
import {Observable, of} from "rxjs";
import {Localized} from "../../../domain_value/localized";
import {BasicSpell} from "../../../domain_value/data/basic_spell";
import {concatMap, map} from "rxjs/operators";
import {DataService} from "../../../service/data";
import {
    CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_ID_MH,
    CONST_AUTO_ATTACK_ID_OH,
    CONST_AUTO_ATTACK_LABEL, CONST_AUTO_ATTACK_LABEL_MH,
    CONST_AUTO_ATTACK_LABEL_OH
} from "../constant/viewer";

@Injectable({
    providedIn: "root",
})
export class SpellService {

    private server_id$: number;

    constructor(
        private dataService: DataService
    ) {
    }

    set_server_id(server_id: number): void {
        this.server_id$ = server_id;
    }

    get_localized_basic_spell(spell_id: number): Observable<Localized<BasicSpell>> {
        if (!this.server_id$)
            return of(this.dataService.unknown_basic_spell);
        if (spell_id === CONST_AUTO_ATTACK_ID)
            return of({
                localization: CONST_AUTO_ATTACK_LABEL,
                base: {
                    id: CONST_AUTO_ATTACK_ID,
                    icon: "inv_sword_04",
                    school: 0
                }
            });
        else if (spell_id === CONST_AUTO_ATTACK_ID_OH)
            return of({
                localization: CONST_AUTO_ATTACK_LABEL_OH,
                base: {
                    id: CONST_AUTO_ATTACK_ID_OH,
                    icon: "inv_sword_04",
                    school: 0
                }
            });
        else if (spell_id === CONST_AUTO_ATTACK_ID_MH)
            return of({
                localization: CONST_AUTO_ATTACK_LABEL_MH,
                base: {
                    id: CONST_AUTO_ATTACK_ID_MH,
                    icon: "inv_sword_04",
                    school: 0
                }
            });
        return this.dataService.get_server_by_id(this.server_id$)
            .pipe(concatMap(server => !server ? of(this.dataService.unknown_basic_spell)
                : this.dataService.get_localized_basic_spell(server.expansion_id, spell_id)));
    }

    get_spell_name(spell_id: number): Observable<string> {
        return this.get_localized_basic_spell(spell_id)
            .pipe(map(spell => spell.localization));
    }
}
