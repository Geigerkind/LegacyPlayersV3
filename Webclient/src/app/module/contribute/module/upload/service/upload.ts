import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class UploadService {
    private static readonly URL_UPLOAD: string = "/live_data_processor/upload";

    constructor(
        private apiService: APIService
    ) {
    }

    upload_file(form_data: FormData): void {
        this.apiService.post_form_data(UploadService.URL_UPLOAD, form_data, () => console.log("SUCCESS"), () => console.log("FAILURE!"));
    }
}
