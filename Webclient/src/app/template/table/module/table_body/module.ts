import {NgModule} from "@angular/core";
import {TableBodyComponent} from "./component/table_body/table_body";
import {CommonModule} from "@angular/common";

@NgModule({
    declarations: [TableBodyComponent],
    imports: [CommonModule],
    exports: [TableBodyComponent]
})
export class TableBodyModule {
}
