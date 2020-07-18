import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {ArenaComponent} from "./component/arena/arena";
import {CommonModule} from "@angular/common";
import {ArenaRouting} from "./routing";
import {TableModule} from "../../../../template/table/module";

@NgModule({
    declarations: [ArenaComponent],
    imports: [
        CommonModule,
        TranslateModule,
        ArenaRouting,
        TableModule
    ],
    exports: [ArenaComponent]
})
export class ArenaModule {
}
