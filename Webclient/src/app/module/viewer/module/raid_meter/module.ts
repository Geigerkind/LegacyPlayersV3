import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidMeterComponent} from "./component/raid_meter/raid_meter";
import {CommonModule} from "@angular/common";
import {SelectInputModule} from "../../../../template/input/select_input/module";

@NgModule({
    declarations: [RaidMeterComponent],
    imports: [
        CommonModule,
        TranslateModule,
        SelectInputModule
    ],
    exports: [RaidMeterComponent]
})
export class RaidMeterModule {
}
