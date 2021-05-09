import {Component, OnChanges, OnInit} from "@angular/core";
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
export class AppComponent implements OnInit, OnChanges {
    public show_cookie_banner = false;
    title = "LegacyPlayers";
    private googleAnalyticsSubscription: Subscription;

    constructor(
        private settingsService: SettingsService,
        private translationService: TranslationService,
        private router: Router
    ) {
        this.settingsService.subscribe("cookieDecisions", item => this.configure_google_analytics(item));
        (window as any).addEventListener("beforeinstallprompt", (e) => () => this.prompt_for_pwa(e));
    }

    ngOnInit(): void {
        this.set_cookie_banner(!this.settingsService.check("cookieDecisions"));
        this.configure_google_analytics(this.settingsService.get("cookieDecisions"));
    }

    ngOnChanges(): void {
        this.configure_google_analytics(this.settingsService.get("cookieDecisions"));
    }

    set_cookie_banner(state: boolean): void {
        this.show_cookie_banner = state;
    }

    private configure_google_analytics(cookieDecisions: any): void {
        if (!gtag)
            return;

        /*
        if (!cookieDecisions || !cookieDecisions.other[0]) {
            if (this.googleAnalyticsSubscription) {
                this.googleAnalyticsSubscription.unsubscribe();
            }
            return;
        }*/

        gtag("config", "UA-107735472-1");
        this.googleAnalyticsSubscription = this.router.events.subscribe(event => {
            if (event instanceof NavigationEnd && (window as any).ga) {
                (window as any).ga("create", "UA-107735472-1", "auto");
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
