import {NgModule} from "@angular/core";
import {TranslateLoader, TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {HttpClient} from "@angular/common/http";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import {TranslationService} from "../../service/translation";
import {ImprintRouting} from "./routing";
import {ImprintComponent} from "./component/imprint/imprint";

export function createTranslateLoader(http: HttpClient) {
    return new TranslateHttpLoader(http, "/assets/i18n/imprint/", ".json");
}

@NgModule({
    declarations: [ImprintComponent],
    imports: [
        CommonModule,
        TranslateModule.forRoot({
            loader: {
                provide: TranslateLoader,
                useFactory: (createTranslateLoader),
                deps: [HttpClient]
            }
        }),
        ImprintRouting
    ],
    exports: [ImprintComponent],
    providers: [
        TranslationService
    ]
})
export class ImprintModule {
}
