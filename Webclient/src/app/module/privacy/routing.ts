import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {PrivacyComponent} from "./component/privacy/privacy";

const routes: Routes = [
    {path: "", component: PrivacyComponent}
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})
export class PrivacyRouting {
}
