import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {ViewerComponent} from "./component/viewer/viewer";
import {CommonModule} from "@angular/common";
import {ViewerRouting} from "./routing";
import {RaidConfigurationMenuModule} from "./module/raid_configuration_menu/module";

@NgModule({
    declarations: [ViewerComponent],
    imports: [
        CommonModule,
        TranslateModule,
        ViewerRouting,
        RaidConfigurationMenuModule
    ],
    exports: [ViewerComponent]
})
export class ViewerModule {
}
