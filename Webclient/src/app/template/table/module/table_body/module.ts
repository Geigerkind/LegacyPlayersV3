import {NgModule} from "@angular/core";
import {TableBodyComponent} from "./component/table_body/table_body";
import {CommonModule} from "@angular/common";
import {BodyTdComponent} from "./component/body_td/body_td";
import {TranslateModule} from "@ngx-translate/core";
import {CaretButtonModule} from "../../../button/caret_button/module";
import {BodyRowComponent} from "./component/body_row/body_row";
import {ResponsiveBodyRowComponent} from "./component/responsive_body_row/responsive_body_row";
import {HeroClassComponent} from "./component/row_components/hero_class/hero_class";

@NgModule({
    declarations: [TableBodyComponent, BodyTdComponent, BodyRowComponent, ResponsiveBodyRowComponent, HeroClassComponent],
    imports: [CommonModule, TranslateModule, CaretButtonModule],
    exports: [TableBodyComponent]
})
export class TableBodyModule {
}
