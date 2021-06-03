import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidBrowserComponent} from "./component/raid_browser/raid_browser";
import {CommonModule} from "@angular/common";
import {MultiSelectModule} from "../../../../../../template/input/multi_select/module";
import {PopupRemoveModule} from "./module/popup_remove/module";

@NgModule({
    declarations: [RaidBrowserComponent],
    imports: [
        CommonModule,
        TranslateModule,
        MultiSelectModule,
        PopupRemoveModule,
    ],
    exports: [RaidBrowserComponent]
})
export class RaidBrowserModule {
}
