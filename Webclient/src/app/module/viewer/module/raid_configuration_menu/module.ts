import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidConfigurationMenuComponent} from "./component/raid_configuration_menu/raid_configuration_menu";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RaidConfigurationMenuComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RaidConfigurationMenuComponent]
})
export class RaidConfigurationMenuModule {
}
