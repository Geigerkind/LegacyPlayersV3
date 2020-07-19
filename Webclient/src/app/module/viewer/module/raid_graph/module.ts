import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidGraphComponent} from "./component/raid_graph/raid_graph";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RaidGraphComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RaidGraphComponent]
})
export class RaidGraphModule {
}
