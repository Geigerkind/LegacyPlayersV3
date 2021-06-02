import {Component} from "@angular/core";
import {TranslationService} from "../../../../service/translation";
import {environment} from "src/environments/environment";
import {APIService} from "../../../../service/api";
import {Meta, Title} from "@angular/platform-browser";

@Component({
    selector: "Privacy",
    templateUrl: "./privacy.html",
    styleUrls: ["./privacy.scss"]
})
export class PrivacyComponent {
    privacyParams: any;

    constructor(
        private translationService: TranslationService,
        private metaService: Meta,
        private titleService: Title
    ) {
        this.titleService.setTitle("LegacyPlayers - Privacy");
        this.metaService.updateTag({name: "description", content: "Privacy statement of Legacyplayers."});

        this.privacyParams = environment.privacy;
    }
}
