import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {GuildViewerComponent} from "./component/guild_viewer/guild_viewer";
import {CommonModule} from "@angular/common";
import {GuildViewerRouting} from "./routing";

@NgModule({
    declarations: [GuildViewerComponent],
    imports: [
        CommonModule,
        TranslateModule,
        GuildViewerRouting
    ],
    exports: [GuildViewerComponent]
})
export class GuildViewerModule {
}
