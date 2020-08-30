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

    upload_file(file_name: string): void {
    }
}
