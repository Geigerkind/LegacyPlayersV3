import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {GuildViewerComponent} from "./component/guild_viewer/guild_viewer";
import {CommonModule} from "@angular/common";
import {GuildViewerRouting} from "./routing";
import {TableModule} from "../../../../template/table/module";
import { HeroClassModule } from 'src/app/template/row_components/hero_class/module';
import {ShowTooltipDirectiveModule} from "../../../../directive/show_tooltip/module";
import {RecentRaidsModule} from "./module/recent_raids/module";
import {GuildRosterModule} from "./module/guild_roster/module";

@NgModule({
    declarations: [GuildViewerComponent],
    imports: [
        CommonModule,
        TranslateModule,
        GuildViewerRouting,
        TableModule,
        HeroClassModule,
        ShowTooltipDirectiveModule,
        GuildRosterModule,
        RecentRaidsModule
    ],
    exports: [GuildViewerComponent]
})
export class GuildViewerModule {
}
