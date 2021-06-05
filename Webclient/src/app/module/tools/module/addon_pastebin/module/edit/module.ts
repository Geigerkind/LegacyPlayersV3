import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {EditComponent} from "./component/edit/edit";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [EditComponent],
    imports: [
        CommonModule,
        TranslateModule
    ],
    exports: [EditComponent]
})
export class EditModule {
}
