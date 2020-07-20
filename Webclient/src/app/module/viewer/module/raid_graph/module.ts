import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidGraphComponent} from "./component/raid_graph/raid_graph";
import {CommonModule} from "@angular/common";
import {MultiSelectModule} from "../../../../template/input/multi_select/module";
import {SelectInputModule} from "../../../../template/input/select_input/module";
import {ChartsModule} from "ng2-charts";

@NgModule({
    declarations: [RaidGraphComponent],
    imports: [
        CommonModule,
        TranslateModule,
        MultiSelectModule,
        SelectInputModule,
        ChartsModule
    ],
    exports: [RaidGraphComponent]
})
export class RaidGraphModule {
}
