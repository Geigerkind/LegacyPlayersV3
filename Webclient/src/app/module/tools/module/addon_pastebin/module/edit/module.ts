import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {EditComponent} from "./component/edit/edit";
import {CommonModule} from "@angular/common";
import {EditRouting} from "./routing";

@NgModule({
    declarations: [EditComponent],
    imports: [
        CommonModule,
        TranslateModule,
        EditRouting
    ],
    exports: [EditComponent]
})
export class EditModule {
}
