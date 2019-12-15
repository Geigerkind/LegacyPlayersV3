import {Component} from "@angular/core";
import {TranslationService} from "../../../../service/translation";
import {environment} from "src/environments/environment";

@Component({
    selector: "Imprint",
    templateUrl: "./imprint.html",
    styleUrls: ["./imprint.scss"]
})
export class ImprintComponent {
    imprintParams: any;

    constructor(private translationService: TranslationService) {
        this.imprintParams = environment.imprint;
    }
}
