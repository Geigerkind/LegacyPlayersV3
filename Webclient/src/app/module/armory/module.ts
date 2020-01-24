import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {ArmoryComponent} from "./component/armory/armory";
import {CommonModule} from "@angular/common";
import {ArmoryRouting} from "./routing";

@NgModule({
    declarations: [ArmoryComponent],
    imports: [
        CommonModule,
        TranslateModule,
        ArmoryRouting
    ],
    exports: [ArmoryComponent]
})
export class ArmoryModule {
}
