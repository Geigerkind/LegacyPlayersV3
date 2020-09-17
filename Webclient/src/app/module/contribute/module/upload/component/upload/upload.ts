import {Component, ElementRef, ViewChild} from "@angular/core";
import {UploadService} from "../../service/upload";

@Component({
    selector: "Upload",
    templateUrl: "./upload.html",
    styleUrls: ["./upload.scss"]
})
export class UploadComponent {

    @ViewChild("upload_file", {static: true}) upload_file: ElementRef;

    server = [
        { value: 6, label_key: "Crystalsong" }
    ];
    selected_server_id: number = this.server[0].value;

    constructor(
        private uploadService: UploadService
    ) {
    }

    upload(): void {
        const formData = new FormData();
        formData.append('server_id', this.selected_server_id.toString());
        formData.append('payload', this.upload_file.nativeElement.files[0]);
        this.uploadService.upload_file(formData);
    }
}
