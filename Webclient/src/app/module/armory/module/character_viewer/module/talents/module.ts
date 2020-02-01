import {NgModule} from "@angular/core";
import {TalentsComponent} from "./component/talents/talents";
import {CommonModule} from "@angular/common";
import {IconModule} from "../../../../../../template/icon/module";

@NgModule({
    declarations: [TalentsComponent],
    imports: [CommonModule, IconModule],
    exports: [TalentsComponent]
})
export class TalentsModule {
}
