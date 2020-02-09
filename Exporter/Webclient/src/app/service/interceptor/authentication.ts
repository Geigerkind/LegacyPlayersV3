import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from "@angular/common/http";
import {Observable, throwError} from "rxjs";
import {Router} from "@angular/router";
import {catchError, map} from "rxjs/operators";
import {SettingsService} from "../settings";
import {APIToken} from "../../domain_value/api_token";

export class AuthenticationInterceptor implements HttpInterceptor {

    constructor(
        private settingsService: SettingsService,
        private routingService: Router
    ) {}

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        if (!req.url.toLowerCase().includes("/api/"))
            return next.handle(req);

        // TODO: REMOVE: DEBUG
        this.settingsService.set("API_TOKEN", {
            "token": "abc",
            "account_id": 5
        });

        // Check if the token exists
        if (!this.settingsService.check("API_TOKEN")) {
            this.routingService.navigate(["/"]); // TODO: Where to redirect to?
            return;
        }
        let api_token: APIToken = this.settingsService.get("API_TOKEN");

        req = req.clone({
            setHeaders: {
                "Content-Type": "application/json",
                "X-Authorization": api_token.token + "|" + api_token.account_id.toString()
            }
        });
        return next.handle(req)
            .pipe(
                map((response: any) => response),
                catchError((failure: any) => {
                    if (failure.status === 401) {
                        this.settingsService.set("API_TOKEN", undefined);
                        this.routingService.navigate(["/"]); // TODO: Where to redirect to?
                    }
                    return throwError(failure);
                })
            );
    }
}
