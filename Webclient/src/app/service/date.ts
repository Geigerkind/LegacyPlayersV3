import {Injectable} from "@angular/core";
import {DatePipe} from "@angular/common";

@Injectable({
    providedIn: "root",
})
export class DateService {

    constructor(private datePipe: DatePipe) {}

    toRPLLLongDate(timestamp_in_ms: any): string {
        return this.datePipe.transform(new Date(Number(timestamp_in_ms)), 'dd.MM.yy hh:mm a', "+0000");
    }

    toRPLLShortDate(timestamp_in_ms: any): string {
        return this.datePipe.transform(new Date(Number(timestamp_in_ms)), 'dd.MM.yy', "+0000");
    }

    toRPLLDateInputDate(timestamp_in_ms: any): string {
        return this.datePipe.transform(new Date(Number(timestamp_in_ms)), 'yyyy-MM-dd', "+0000");
    }

}
