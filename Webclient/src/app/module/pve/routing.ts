import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {PveComponent} from "./component/pve/pve";

const routes: Routes = [
    {path: "", component: PveComponent}
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class PveRouting {
}
