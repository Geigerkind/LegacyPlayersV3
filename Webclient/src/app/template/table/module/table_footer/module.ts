import {NgModule} from "@angular/core";
import {TableFooterComponent} from "./component/table_footer/table_footer";
import {CommonModule} from "@angular/common";
import {TranslateModule} from "@ngx-translate/core";

@NgModule({
    declarations: [TableFooterComponent],
    imports: [CommonModule, TranslateModule],
    exports: [TableFooterComponent]
})
export class TableFooterModule {
}
