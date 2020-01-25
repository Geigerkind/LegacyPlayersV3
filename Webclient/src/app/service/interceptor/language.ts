import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from "@angular/common/http";
import {Observable} from "rxjs";

export class LanguageInterceptor implements HttpInterceptor {

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        if (!req.url.toLowerCase().includes("/api/"))
            return next.handle(req);

        req = req.clone({
            setHeaders: {
                "Content-Type": "application/json",
                "X-Language": "en"
            }
        });
        return next.handle(req);
    }
}
