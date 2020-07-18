import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {SkirmishComponent} from "./component/skirmish/skirmish";
import {CommonModule} from "@angular/common";
import {SkirmishRouting} from "./routing";
import {TableModule} from "../../../../template/table/module";

@NgModule({
    declarations: [SkirmishComponent],
    imports: [
        CommonModule,
        TranslateModule,
        SkirmishRouting,
        TableModule
    ],
    exports: [SkirmishComponent]
})
export class SkirmishModule {
}
