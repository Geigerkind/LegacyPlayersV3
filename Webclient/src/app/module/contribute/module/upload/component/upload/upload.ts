import {Component} from "@angular/core";
import {UploadService} from "../../service/upload";

@Component({
    selector: "Upload",
    templateUrl: "./upload.html",
    styleUrls: ["./upload.scss"]
})
export class UploadComponent {

    server = [
        { value: 6, label_key: "Crystalsong" }
    ];

    constructor(
        private uploadService: UploadService
    ) {
    }
}
