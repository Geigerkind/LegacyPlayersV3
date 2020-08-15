import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidDetailTableComponent} from "./component/raid_detail_table/raid_detail_table";
import {CommonModule} from "@angular/common";
import {SelectInputModule} from "../../../../template/input/select_input/module";

@NgModule({
    declarations: [RaidDetailTableComponent],
    imports: [
        CommonModule,
        TranslateModule,
        SelectInputModule
    ],
    exports: [RaidDetailTableComponent]
})
export class RaidDetailTableModule {
}
