import {Component, ElementRef, ViewChild} from "@angular/core";
import {UploadService} from "../../service/upload";
import {Severity} from "../../../../../../domain_value/severity";
import {APIService} from "../../../../../../service/api";
import {NotificationService} from "../../../../../../service/notification";
import {DateService} from "../../../../../../service/date";

@Component({
    selector: "Upload",
    templateUrl: "./upload.html",
    styleUrls: ["./upload.scss"]
})
export class UploadComponent {

    @ViewChild("upload_file", {static: true}) upload_file: ElementRef;
    disableSubmit = false;

    server = [
        {value: 6, label_key: "Crystalsong"}
    ];
    selected_server_id: number = this.server[0].value;
    selected_start_date: string;
    selected_end_date: string;

    constructor(
        private uploadService: UploadService,
        private notification_service: NotificationService,
        private date_service: DateService
    ) {
        const start = new Date();
        start.setUTCHours(1, 0, 0, 0);
        const end = new Date();
        end.setUTCHours(23, 0, 0, 0);

        this.selected_start_date = this.date_service.toRPLLLongDate(start);
        this.selected_end_date = this.date_service.toRPLLLongDate(end);
    }

    upload(): void {
        if (!this.disableSubmit) {
            this.disableSubmit = true;
            this.notification_service.propagate(Severity.Info, "Uploading...");
            const formData = new FormData();
            formData.append('server_id', this.selected_server_id.toString());
            formData.append('start_time', this.selected_start_date);
            formData.append('end_time', this.selected_end_date);
            formData.append('payload', this.upload_file.nativeElement.files[0]);
            this.uploadService.upload_file(formData, () => {
                this.notification_service.propagate(Severity.Success, "Your log has been uploaded!");
                this.disableSubmit = false;
            }, () => {
                this.notification_service.propagate(Severity.Error, "Your log failed to upload!");
                this.disableSubmit = false;
            });
        }
    }
}
