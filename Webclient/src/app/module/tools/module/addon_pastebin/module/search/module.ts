import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {SearchComponent} from "./component/search/search";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [SearchComponent],
    imports: [
        CommonModule,
        TranslateModule,
    ],
    exports: [SearchComponent]
})
export class SearchModule {
}
