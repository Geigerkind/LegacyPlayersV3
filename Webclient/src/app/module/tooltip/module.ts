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
import {DeathsOverviewModule} from "../viewer/module/raid_meter/module/deaths_overview/module";
import {ViewerEventLogTooltipModule} from "./module/viewer_event_log_tooltip/module";
import {UnAuraOverviewModule} from "../viewer/module/raid_meter/module/un_aura_overview/module";
import {ViewerGraphTooltipModule} from "./module/viewer_graph_tooltip/module";

@NgModule({
    declarations: [TooltipComponent],
    imports: [
        CommonModule,
        CharacterTooltipModule,
        ItemTooltipModule,
        GuildTooltipModule,
        MeterAbilitiesTooltipModule,
        MeterDetailsTooltipModule,
        TranslateModule,
        DeathsOverviewModule,
        ViewerEventLogTooltipModule,
        UnAuraOverviewModule,
        ViewerGraphTooltipModule
    ],
    exports: [TooltipComponent],
    providers: [TooltipService]
})
export class TooltipModule {
}
