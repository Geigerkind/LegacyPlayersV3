import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {ArmoryComponent} from "./component/armory/armory";

const routes: Routes = [{
    path: "", component: ArmoryComponent, children: [
        {path: "", loadChildren: () => import("./module/search/module").then(m => m.SearchModule)},
        {path: "character/:character_id", loadChildren: () => import("./module/character_viewer/module").then(m => m.CharacterViewerModule)},
        {path: "character/:character_id/:character_history_id", loadChildren: () => import("./module/character_viewer/module").then(m => m.CharacterViewerModule)},
    ]
}];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class ArmoryRouting {
}
