import {Component} from "@angular/core";
import {TranslationService} from "../../../../service/translation";
import {environment} from "src/environments/environment";

@Component({
    selector: "Privacy",
    templateUrl: "./privacy.html",
    styleUrls: ["./privacy.scss"]
})
export class PrivacyComponent {
    privacyParams: any;

    constructor(private translationService: TranslationService) {
        this.privacyParams = environment.privacy;
    }
}
