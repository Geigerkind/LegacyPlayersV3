import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {ArmoryComponent} from "./component/armory/armory";

const routes: Routes = [{
    path: "", component: ArmoryComponent, children: [
        {path: "", loadChildren: () => import("./module/search/module").then(m => m.SearchModule)}
    ]
}];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class ArmoryRouting {
}
