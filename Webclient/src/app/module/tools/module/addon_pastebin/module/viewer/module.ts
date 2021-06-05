import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {ViewerComponent} from "./component/viewer/viewer";
import {CommonModule} from "@angular/common";
import {ViewerRouting} from "./routing";

@NgModule({
    declarations: [ViewerComponent],
    imports: [
        CommonModule,
        TranslateModule,
        ViewerRouting
    ],
    exports: [ViewerComponent]
})
export class ViewerModule {
}
