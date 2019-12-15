import {BrowserModule} from "@angular/platform-browser";
import {NgModule} from "@angular/core";

import {AppRouting} from "./routing";
import {AppComponent} from "./component/app/app";

import {HttpClient, HttpClientModule} from "@angular/common/http";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import {TranslateLoader, TranslateModule} from "@ngx-translate/core";
import {FooterBarModule} from "./module/footer_bar/module";
import {NavigationBarModule} from "./module/navigation_bar/module";
import {CookieBannerComponent} from "./module/cookie_banner/component/cookie_banner/cookie_banner";
import {ReactiveComponentLoaderModule} from "@wishtack/reactive-component-loader";
import {SettingsService} from "./service/settings";
import {NotificationListModule} from "./module/notification_list/module";
import {NotificationService} from "./service/notification";
import {RouterLoadingBarModule} from "./module/router_loading_bar/module";
import {TranslationService} from "./service/translation";
import {APIService} from "./service/api";
import {LoadingBarService} from "./service/loading_bar";

export function createTranslateLoader(http: HttpClient) {
    return new TranslateHttpLoader(http, "./assets/i18n/", ".json");
}

@NgModule({
    declarations: [AppComponent],
    imports: [
        BrowserModule,
        AppRouting,
        NotificationListModule,
        NavigationBarModule,
        RouterLoadingBarModule,
        FooterBarModule,
        HttpClientModule,
        TranslateModule.forRoot({
            loader: {
                provide: TranslateLoader,
                useFactory: (createTranslateLoader),
                deps: [HttpClient]
            }
        }),
        ReactiveComponentLoaderModule.forRoot(),
        ReactiveComponentLoaderModule.withModule({
            moduleId: "cookie_banner",
            loadChildren: "./module/cookie_banner/module#CookieBannerModule"
        })
    ],
    providers: [
        SettingsService,
        NotificationService,
        TranslationService,
        LoadingBarService,
        APIService
    ],
    bootstrap: [AppComponent],
    entryComponents: [CookieBannerComponent]
})
export class AppModule {
}
