import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {UploadsComponent} from "./component/uploads/uploads";
import {CommonModule} from "@angular/common";
import {UploadsRouting} from "./routing";
import {TableModule} from "../../../../template/table/module";
import {InstanceMapModule} from "../../../../template/row_components/instance_map/module";

@NgModule({
    declarations: [UploadsComponent],
    imports: [
        CommonModule,
        TranslateModule,
        UploadsRouting,
        TableModule,
        InstanceMapModule
    ],
    exports: [UploadsComponent]
})
export class UploadsModule {
}
