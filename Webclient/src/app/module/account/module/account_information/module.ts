import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {AccountInformationComponent} from "./component/account_information/account_information";
import {CommonModule} from "@angular/common";
import {AccountInformationRouting} from "./routing";
import {AccountInformationService} from "./service/account_information";
import {ConfirmButtonModule} from "../../../../template/button/confirm_button/module";

@NgModule({
    declarations: [AccountInformationComponent],
    imports: [
        CommonModule,
        TranslateModule,
        AccountInformationRouting,
        ConfirmButtonModule
    ],
    exports: [AccountInformationComponent],
    providers: [AccountInformationService]
})
export class AccountInformationModule {
}
