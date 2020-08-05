import {NgModule} from "@angular/core";
import {CharacterViewerComponent} from "./component/character_viewer/character_viewer";
import {CommonModule} from "@angular/common";
import {CharacterViewerRouting} from "./routing";
import {CharacterItemsModule} from "./module/character_items/module";
import {ProfessionModule} from "./module/profession/module";
import {TalentsModule} from "./module/talents/module";
import {StatsModule} from "./module/stats/module";
import {RankingTableModule} from "./module/ranking_table/module";

@NgModule({
    declarations: [CharacterViewerComponent],
    imports: [
        CommonModule,
        CharacterViewerRouting,
        CharacterItemsModule,
        ProfessionModule,
        TalentsModule,
        StatsModule,
        RankingTableModule
    ],
    exports: [CharacterViewerComponent]
})
export class CharacterViewerModule {
}
