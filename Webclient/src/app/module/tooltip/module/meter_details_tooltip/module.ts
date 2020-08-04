import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {MeterDetailsTooltipComponent} from "./component/meter_details_tooltip/meter_details_tooltip";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [MeterDetailsTooltipComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [MeterDetailsTooltipComponent]
})
export class MeterDetailsTooltipModule {
}
