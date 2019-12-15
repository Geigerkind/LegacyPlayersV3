import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {LogOutGuard} from "./guard/logout";
import {AuthenticateGuard} from "./guard/authenticate";
import {SignedInGuard} from "./guard/signed_in";

const routes: Routes = [
    {path: "", loadChildren: () => import("./module/home/module").then(m => m.HomeModule)},
    {
        path: "account",
        loadChildren: () => import("./module/account/module").then(m => m.AccountModule),
        canLoad: [AuthenticateGuard]
    },
    {
        path: "login",
        loadChildren: () => import("./module/login/module").then(m => m.LoginModule),
        canLoad: [SignedInGuard]
    },
    {path: "logout", children: [], canActivate: [LogOutGuard]},
    {
        path: "sign_up",
        loadChildren: () => import("./module/sign_up/module").then(m => m.SignUpModule),
        canLoad: [SignedInGuard]
    },
    {
        path: "reset_password",
        loadChildren: () => import("./module/reset_password/module").then(m => m.ResetPasswordModule),
        canLoad: [SignedInGuard]
    },
    {
        path: "confirm/:type/:confirm_id",
        loadChildren: () => import("./module/confirm/module").then(m => m.ConfirmModule),
    },
    {path: "privacy", loadChildren: () => import("./module/privacy/module").then(m => m.PrivacyModule)},
    {path: "imprint", loadChildren: () => import("./module/imprint/module").then(m => m.ImprintModule)}
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRouting {
}
