import {NgModule} from "@angular/core";
import {CharacterViewerComponent} from "./component/character_viewer/character_viewer";
import {CommonModule} from "@angular/common";
import {CharacterViewerRouting} from "./routing";
import {CharacterItemsModule} from "./module/character_items/module";

@NgModule({
    declarations: [CharacterViewerComponent],
    imports: [
        CommonModule,
        CharacterViewerRouting,
        CharacterItemsModule
    ],
    exports: [CharacterViewerComponent]
})
export class CharacterViewerModule {
}
