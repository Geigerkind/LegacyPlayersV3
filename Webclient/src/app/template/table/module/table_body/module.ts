import {NgModule} from "@angular/core";
import {TableBodyComponent} from "./component/table_body/table_body";
import {CommonModule} from "@angular/common";
import {BodyTdComponent} from "./component/body_td/body_td";
import {TranslateModule} from "@ngx-translate/core";

@NgModule({
    declarations: [TableBodyComponent, BodyTdComponent],
    imports: [CommonModule, TranslateModule],
    exports: [TableBodyComponent]
})
export class TableBodyModule {
}
