import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidMeterComponent} from "./component/raid_meter/raid_meter";
import {CommonModule} from "@angular/common";
import {SelectInputModule} from "../../../../template/input/select_input/module";
import {RouterModule} from "@angular/router";
import {ShowTooltipDirectiveModule} from "../../../../directive/show_tooltip/module";
import {MeterGraphModule} from "../../../../template/meter_graph/module";

@NgModule({
    declarations: [RaidMeterComponent],
    imports: [
        CommonModule,
        TranslateModule,
        SelectInputModule,
        RouterModule,
        ShowTooltipDirectiveModule,
        MeterGraphModule
    ],
    exports: [RaidMeterComponent]
})
export class RaidMeterModule {
}
