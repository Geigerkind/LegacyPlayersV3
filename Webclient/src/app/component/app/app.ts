import {Component, OnInit} from "@angular/core";
import {ComponentLocation} from "@wishtack/reactive-component-loader";
import {NavigationEnd, Router} from "@angular/router";
import {Subscription} from "rxjs";
import {SettingsService} from "../../service/settings";
import {TranslationService} from "../../service/translation";

declare var gtag;

@Component({
    selector: "root",
    templateUrl: "./app.html",
    styleUrls: ["./app.scss"]
})
export class AppComponent implements OnInit {
    private static readonly PWA_PROMPT_TIME = 30000;
    private static show_cookie_banner = false;
    title = "Webclient";
    location: ComponentLocation = null;
    private googleAnalyticsSubscription: Subscription;
    private cookie_banner: ComponentLocation = {
        moduleId: "cookie_banner",
        selector: "CookieBanner"
    };

    constructor(private translationService: TranslationService,
                private settingsService: SettingsService,
                private router: Router) {
        this.settingsService.subscribe("cookieDecisions", item => this.configure_google_analytics(item));
        (window as any).addEventListener("beforeinstallprompt", (e) => setTimeout((evnt) => this.prompt_for_pwa(evnt), AppComponent.PWA_PROMPT_TIME, e));
    }

    get isCookieBannerVisible(): boolean {
        return AppComponent.show_cookie_banner;
    }

    ngOnInit(): void {
        this.set_cookie_banner(!this.settingsService.check("cookieDecisions"));
        this.configure_google_analytics(this.settingsService.get("cookieDecisions"));
    }

    set_cookie_banner(state: boolean): void {
        AppComponent.show_cookie_banner = state;
        if (state && this.location === null)
            this.location = this.cookie_banner;
    }

    private configure_google_analytics(cookieDecisions: any): void {
        if (!cookieDecisions || !cookieDecisions.other[0]) {
            if (this.googleAnalyticsSubscription) {
                this.googleAnalyticsSubscription.unsubscribe();
            }
            return;
        }

        gtag("config", "UA-152539504-1");
        this.googleAnalyticsSubscription = this.router.events.subscribe(event => {
            if (event instanceof NavigationEnd && (window as any).ga) {
                (window as any).ga("create", "UA-152539504-1", "auto");
                (window as any).ga("set", "anonymizeIp", true);
                (window as any).ga("set", "allowAdFeatures", false);
                (window as any).ga("set", "page", event.urlAfterRedirects);
                (window as any).ga("send", "pageview");
            }
        });
    }

    private prompt_for_pwa(e: any): void {
        if (this.settingsService.check("PWA_PROMPT"))
            return;
        e.prompt();
        this.settingsService.set("PWA_PROMPT", true);
    }
}
