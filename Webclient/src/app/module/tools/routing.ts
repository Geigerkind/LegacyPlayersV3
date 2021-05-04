import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {ToolsComponent} from "./component/tools/tools";

const routes: Routes = [{
    path: "", component: ToolsComponent, children: [
        {path: "", redirectTo: "/tools/talents", pathMatch: "full" },
        {
            path: "talents",
            loadChildren: () => import("./module/talent_viewer/module").then(m => m.TalentViewerModule)
        },
        {
            path: "talents/:expansion_id/:hero_class_id/:spec_1/:spec_2/:spec_3",
            loadChildren: () => import("./module/talent_viewer/module").then(m => m.TalentViewerModule)
        }
    ]
}];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class ToolsRouting {
}
