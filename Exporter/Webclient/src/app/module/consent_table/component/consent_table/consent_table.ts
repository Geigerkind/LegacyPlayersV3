import {Component} from "@angular/core";
import {ConsentService} from "../../service/consent";

@Component({
    selector: "ConsentTable",
    templateUrl: "./consent_table.html",
    styleUrls: ["./consent_table.scss"]
})
export class ConsentTableComponent {

    sampleCharacters: Array<{ character_id: number, hero_class_id: number, name: string, consent: boolean }> = [
        { character_id: 1, hero_class_id: 2, name: "Peter", consent: true },
        { character_id: 2, hero_class_id: 3, name: "PeterPan", consent: true },
        { character_id: 3, hero_class_id: 4, name: "PanPeter", consent: false }
    ];

    constructor(
        private consentService: ConsentService
    ) {
    }

    handleConsent(character_id: number, state: boolean) {
        if (state) this.consentService.give_character_consent(character_id, () => {});
        else this.consentService.withdraw_character_consent(character_id, () => {});
    }

}
