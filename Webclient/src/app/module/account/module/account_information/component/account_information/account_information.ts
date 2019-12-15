import {Component} from "@angular/core";
import {AccountInformation} from "../../../../domain_value/account_information";
import {AccountInformationService} from "../../service/account_information";

@Component({
    selector: "AccountInformation",
    templateUrl: "./account_information.html",
    styleUrls: ["./account_information.scss"]
})
export class AccountInformationComponent {
    initialLoading: boolean = true;
    disabledSubmit: boolean = false;
    accountInformation: AccountInformation;

    constructor(private accountInformationService: AccountInformationService) {
        this.accountInformationService.get((acc_info) => this.on_success(acc_info));
    }

    on_success(account_information: AccountInformation): void {
        this.accountInformation = account_information;
        this.initialLoading = false;
    }

    resend_confirmation(): void {
        this.accountInformationService.resend_confirmation(() => this.disabledSubmit = false);
    }
}
