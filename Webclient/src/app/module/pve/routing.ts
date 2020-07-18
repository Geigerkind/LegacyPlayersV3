import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {PveComponent} from "./component/pve/pve";

const routes: Routes = [
    {
        path: "", component: PveComponent, children: [
            {path: "", loadChildren: () => import("./module/search/module").then(m => m.SearchModule)},
        ]
    }
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class PveRouting {
}
