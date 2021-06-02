import {Component} from "@angular/core";
import {TranslationService} from "../../../../service/translation";
import {environment} from "src/environments/environment";
import {Meta, Title} from "@angular/platform-browser";

@Component({
    selector: "Imprint",
    templateUrl: "./imprint.html",
    styleUrls: ["./imprint.scss"]
})
export class ImprintComponent {
    imprintParams: any;

    constructor(
        private translationService: TranslationService,
        private metaService: Meta,
        private titleService: Title
    ) {
        this.titleService.setTitle("LegacyPlayers - Imprint");
        this.metaService.updateTag({name: "description", content: "Imprint of Legacyplayers."});
        this.imprintParams = environment.imprint;
    }
}
