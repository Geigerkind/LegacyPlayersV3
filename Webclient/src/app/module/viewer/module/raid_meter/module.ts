import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidMeterComponent} from "./component/raid_meter/raid_meter";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RaidMeterComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RaidMeterComponent]
})
export class RaidMeterModule {
}
