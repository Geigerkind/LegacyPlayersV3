import {Injectable} from "@angular/core";
import {HttpClient, HttpErrorResponse, HttpHeaders} from "@angular/common/http";
import {SettingsService} from "./settings";
import {NotificationService} from "./notification";
import {Severity} from "../domain_value/severity";
import {Router} from "@angular/router";
import {LoadingBarService} from "./loading_bar";
import {APIFailure} from "../domain_value/api_failure";

@Injectable({
    providedIn: "root",
})
export class APIService {
    private static readonly API_PREFIX: string = "/API";

    constructor(private httpClient: HttpClient,
                private settingsService: SettingsService,
                private notificationService: NotificationService,
                private routingService: Router,
                private loadingBarService: LoadingBarService) {
    }

    get_auth<T>(url: string, on_success?: (T) => void, on_failure?: (any) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.get<T>(APIService.API_PREFIX + url, {headers: this.setAuthHeader(this.httpHeaderFactory())})
            .toPromise()
            .then(response => {
                if (!!on_success)
                    on_success.call(on_success, response);
            })
            .catch(reason => {
                const failure = this.handleFailure(reason);
                if (!!on_failure)
                    on_failure.call(on_failure, failure);
            })
            .finally(() => this.loadingBarService.decrementCounter());
    }

    get<T>(url: string, on_success?: (T) => void, on_failure?: (any) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.get<T>(APIService.API_PREFIX + url, {headers: this.httpHeaderFactory()})
            .toPromise()
            .then(response => {
                if (!!on_success)
                    on_success.call(on_success, response);
            })
            .catch(reason => {
                const failure = this.handleFailure(reason);
                if (!!on_failure)
                    on_failure.call(on_failure, failure);
            })
            .finally(() => this.loadingBarService.decrementCounter());
    }

    post_auth<T1, T2>(url: string, body: T2, on_success?: (T1) => void, on_failure?: (any) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.post<T1>(APIService.API_PREFIX + url, JSON.stringify(body), {headers: this.setAuthHeader(this.httpHeaderFactory())})
            .toPromise()
            .then(response => {
                if (!!on_success)
                    on_success.call(on_success, response);
            })
            .catch(reason => {
                const failure = this.handleFailure(reason);
                if (!!on_failure)
                    on_failure.call(on_failure, failure);
            })
            .finally(() => this.loadingBarService.decrementCounter());
    }

    post<T1, T2>(url: string, body: T2, on_success?: (T1) => void, on_failure?: (any) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.post<T1>(APIService.API_PREFIX + url, JSON.stringify(body), {headers: this.httpHeaderFactory()})
            .toPromise()
            .then(response => {
                if (!!on_success)
                    on_success.call(on_success, response);
            })
            .catch(reason => {
                const failure = this.handleFailure(reason);
                if (!!on_failure)
                    on_failure.call(on_failure, failure);
            })
            .finally(() => this.loadingBarService.decrementCounter());
    }

    delete_auth<T1, T2>(url: string, body: any, on_success?: (T1) => void, on_failure?: (any) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.request<T1>("delete", APIService.API_PREFIX + url, {
            body: JSON.stringify(body),
            headers: this.setAuthHeader(this.httpHeaderFactory())
        })
            .toPromise()
            .then(response => {
                if (!!on_success)
                    on_success.call(on_success, response);
            })
            .catch(reason => {
                const failure = this.handleFailure(reason);
                if (!!on_failure)
                    on_failure.call(on_failure, failure);
            })
            .finally(() => this.loadingBarService.decrementCounter());
    }

    private httpHeaderFactory(): HttpHeaders {
        return new HttpHeaders()
            .set("Content-Type", "application/json");
    }

    private setAuthHeader(headers: HttpHeaders): HttpHeaders {
        let api_token = "";
        if (this.settingsService.check("API_TOKEN"))
            api_token = this.settingsService.get("API_TOKEN").token;
        return headers.set("Authorization", api_token);
    }

    private handleFailure(reason: HttpErrorResponse): APIFailure {
        if (reason.status < 400)
            return;

        // Token invalid
        if (reason.status === 401) {
            this.settingsService.set("API_TOKEN", undefined);
            this.routingService.navigate(["/login"]);
            return;
        }

        const api_failure: APIFailure = {
            status: reason.status,
            translation: "serverResponses." + reason.status,
            arguments: {
                arg1: reason.error
            }
        };
        this.notificationService.propagate(Severity.Error, api_failure.translation, api_failure.arguments);
        return api_failure;
    }
}
