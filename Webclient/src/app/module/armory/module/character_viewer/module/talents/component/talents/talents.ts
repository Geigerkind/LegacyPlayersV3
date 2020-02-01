import {Component, Input} from "@angular/core";
import {TalentSpecializationDto} from "../../../../domain_value/talent_specialization_dto";

@Component({
    selector: "Talents",
    templateUrl: "./talents.html",
    styleUrls: ["./talents.scss"]
})
export class TalentsComponent {

    @Input() talents: TalentSpecializationDto;

    // TODO
    getTalentBreakdown(): string {
        return '0/8/53';
    }

}
