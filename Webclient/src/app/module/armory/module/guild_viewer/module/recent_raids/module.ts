import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {RecentRaidsComponent} from "./component/recent_raids/recent_raids";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [RecentRaidsComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [RecentRaidsComponent]
})
export class RecentRaidsModule {
}
