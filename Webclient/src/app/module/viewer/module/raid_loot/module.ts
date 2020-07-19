import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidLootComponent} from "./component/raid_loot/raid_loot";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RaidLootComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RaidLootComponent]
})
export class RaidLootModule {
}
