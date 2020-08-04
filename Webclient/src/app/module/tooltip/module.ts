import {NgModule} from "@angular/core";
import {TooltipComponent} from "./component/tooltip/tooltip";
import {CommonModule} from "@angular/common";
import {CharacterTooltipModule} from "./module/character_tooltip/module";
import {TooltipService} from "./service/tooltip";
import {TranslateModule} from "@ngx-translate/core";
import {ItemTooltipModule} from "./module/item_tooltip/module";
import {GuildTooltipModule} from "./module/guild_tooltip/module";
import {MeterAbilitiesTooltipModule} from "./module/meter_abilities_tooltip/module";
import {MeterDetailsTooltipModule} from "./module/meter_details_tooltip/module";

@NgModule({
    declarations: [TooltipComponent],
    imports: [
        CommonModule,
        CharacterTooltipModule,
        ItemTooltipModule,
        GuildTooltipModule,
        MeterAbilitiesTooltipModule,
        MeterDetailsTooltipModule,
        TranslateModule
    ],
    exports: [TooltipComponent],
    providers: [TooltipService]
})
export class TooltipModule {
}
