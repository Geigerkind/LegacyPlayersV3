import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CharacterViewerComponent} from "./component/character_viewer/character_viewer";
import {CommonModule} from "@angular/common";
import {CharacterViewerRouting} from "./routing";
import {CharacterItemsComponent} from "./component/character_items/character_items";
import {IconModule} from "../../../../template/icon/module";
import {SelectInputModule} from "../../../../template/input/select_input/module";
import {ItemIconModule} from "../../../../template/item_icon/module";
import {CharacterItemComponent} from "./component/character_items/character_item/character_item";

@NgModule({
    declarations: [CharacterViewerComponent, CharacterItemsComponent, CharacterItemComponent],
    imports: [
        CommonModule,
        TranslateModule,
        CharacterViewerRouting,
        IconModule,
        SelectInputModule,
        ItemIconModule
    ],
    exports: [CharacterViewerComponent]
})
export class CharacterViewerModule {
}
