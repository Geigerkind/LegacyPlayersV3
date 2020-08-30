import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {UploadComponent} from "./component/upload/upload";
import {CommonModule} from "@angular/common";
import {UploadRouting} from "./routing";
import {SelectInputModule} from "../../../../template/input/select_input/module";

@NgModule({
    declarations: [UploadComponent],
    imports: [
        CommonModule,
        TranslateModule,
        UploadRouting,
        SelectInputModule
    ],
    exports: [UploadComponent]
})
export class UploadModule {
}
