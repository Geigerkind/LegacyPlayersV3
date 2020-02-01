import {Component, Input} from "@angular/core";
import {TalentSpecializationDto} from "../../../../domain_value/talent_specialization_dto";

@Component({
    selector: "Talents",
    templateUrl: "./talents.html",
    styleUrls: ["./talents.scss"]
})
export class TalentsComponent {

    @Input() talents: TalentSpecializationDto;

    getTalentBreakdown(): string {
        return this.talents.description
            .split('|')
            .map(spec => [...spec]
                .map(talent => Number(talent))
            .reduce((acc, item) => acc += item))
            .join('/');
    }

}
