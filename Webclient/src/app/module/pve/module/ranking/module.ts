import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RankingComponent} from "./component/ranking/ranking";
import {CommonModule} from "@angular/common";
import {RankingRouting} from "./routing";
import {MeterGraphModule} from "../../../../template/meter_graph/module";
import {SelectInputModule} from "../../../../template/input/select_input/module";

@NgModule({
    declarations: [RankingComponent],
    imports: [
        CommonModule,
        TranslateModule,
        RankingRouting,
        MeterGraphModule,
        SelectInputModule
    ],
    exports: [RankingComponent]
})
export class RankingModule {
}
