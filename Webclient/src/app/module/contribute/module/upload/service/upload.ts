import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {NotificationService} from "../../../../../service/notification";
import {Severity} from "../../../../../domain_value/severity";

@Injectable({
    providedIn: "root",
})
export class UploadService {
    private static readonly URL_UPLOAD: string = "/live_data_processor/upload";

    constructor(
        private apiService: APIService,
        private notification_service: NotificationService
    ) {
    }

    upload_file(form_data: FormData): void {
        this.notification_service.propagate(Severity.Info, "Uploading...");
        this.apiService.post_form_data(UploadService.URL_UPLOAD, form_data, () => {
            this.notification_service.propagate(Severity.Success, "Your log has been uploaded!");
        }, () => {
            this.notification_service.propagate(Severity.Error, "Your log failed to upload!");
        });
    }
}
