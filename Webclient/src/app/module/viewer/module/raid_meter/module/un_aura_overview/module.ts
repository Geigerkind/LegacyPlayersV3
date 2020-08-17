import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {UnAuraOverviewComponent} from "./component/un_aura_overview/un_aura_overview";
import {CommonModule} from "@angular/common";
import {RouterModule} from "@angular/router";
import {ShowTooltipDirectiveModule} from "../../../../../../directive/show_tooltip/module";

@NgModule({
    declarations: [UnAuraOverviewComponent],
    imports: [
        CommonModule,
        TranslateModule,
        RouterModule,
        ShowTooltipDirectiveModule
    ],
    exports: [UnAuraOverviewComponent]
})
export class UnAuraOverviewModule {
}
