import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class UploadsService {
    private static readonly URL_RAIDS_META: string = "/instance/meta_search/raids/by_member_id";
    private static readonly URL_DELETE_RAID: string = "/instance/delete";

    constructor(
        private apiService: APIService
    ) {
    }

    search_raids(filter: any, on_success: any, on_failure: any): void {
        this.apiService.post(UploadsService.URL_RAIDS_META, filter,
            (result) => on_success.call(on_success, result), on_failure);
    }

    delete_raid(instance_meta_id: number, on_success: any): void {
        this.apiService.delete(UploadsService.URL_DELETE_RAID, instance_meta_id, (result) => on_success.call(on_success, result));
    }
}
