import {NgModule} from "@angular/core";
import {TranslateLoader, TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {PrivacyComponent} from "./component/privacy/privacy";
import {PrivacyRouting} from "./routing";
import {HttpClient} from "@angular/common/http";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import {TranslationService} from "../../service/translation";

export function createTranslateLoader(http: HttpClient) {
    return new TranslateHttpLoader(http, "/assets/i18n/privacy/", ".json");
}

@NgModule({
    declarations: [PrivacyComponent],
    imports: [
        CommonModule,
        TranslateModule.forRoot({
            loader: {
                provide: TranslateLoader,
                useFactory: (createTranslateLoader),
                deps: [HttpClient]
            }
        }),
        PrivacyRouting
    ],
    exports: [PrivacyComponent],
    providers: [
        TranslationService
    ]
})
export class PrivacyModule {
}
