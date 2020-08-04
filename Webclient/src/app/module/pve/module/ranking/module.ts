import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RankingComponent} from "./component/ranking/ranking";
import {CommonModule} from "@angular/common";
import {RankingRouting} from "./routing";

@NgModule({
    declarations: [RankingComponent],
    imports: [
        CommonModule,
        TranslateModule,
        RankingRouting
    ],
    exports: [RankingComponent]
})
export class RankingModule {
}
