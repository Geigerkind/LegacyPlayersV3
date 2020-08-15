import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidConfigurationMenuComponent} from "./component/raid_configuration_menu/raid_configuration_menu";
import {CommonModule} from "@angular/common";
import {MultiSelectModule} from "../../../../template/input/multi_select/module";
import {GeneralInputModule} from "../../../../template/input/general_input/module";
import {ClickOutsideDirectiveModule} from "../../../../directive/click_outside/module";

@NgModule({
    declarations: [RaidConfigurationMenuComponent],
    imports: [
        CommonModule,
        TranslateModule,
        MultiSelectModule,
        GeneralInputModule,
        ClickOutsideDirectiveModule
    ],
    exports: [RaidConfigurationMenuComponent]
})
export class RaidConfigurationMenuModule {
}
