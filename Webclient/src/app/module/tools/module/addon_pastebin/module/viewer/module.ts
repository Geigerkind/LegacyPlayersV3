import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {ViewerComponent} from "./component/viewer/viewer";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [ViewerComponent],
    imports: [
        CommonModule,
        TranslateModule,
    ],
    exports: [ViewerComponent]
})
export class ViewerModule {
}
