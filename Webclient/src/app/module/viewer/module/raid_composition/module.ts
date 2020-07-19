import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RaidCompositionComponent} from "./component/raid_composition/raid_composition";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RaidCompositionComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RaidCompositionComponent]
})
export class RaidCompositionModule {
}
