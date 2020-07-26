import {Component} from "@angular/core";
import {InstanceDataService} from "../../../../service/instance_data";
import {InstanceViewerParticipants} from "../../../../domain_value/instance_viewer_participants";
import {Role} from "../../../../domain_value/role";

@Component({
    selector: "RaidComposition",
    templateUrl: "./raid_composition.html",
    styleUrls: ["./raid_composition.scss"]
})
export class RaidCompositionComponent {

    show_tanks: boolean = false;
    show_healers: boolean = false;
    show_dps: boolean = false;

    tanks: Array<Array<InstanceViewerParticipants>> = [];
    healer: Array<Array<InstanceViewerParticipants>> = [];
    dps: Array<Array<InstanceViewerParticipants>> = [];

    constructor(
        private instanceDataService: InstanceDataService
    ) {
        instanceDataService.participants.subscribe(participants => {
            this.tanks = this.create_participant_array(participants, Role.Tank);
            this.healer = this.create_participant_array(participants, Role.Healer);
            this.dps = this.create_participant_array(participants, Role.Dps);
        });
    }

    private create_participant_array(participants: Array<InstanceViewerParticipants>, role: Role): Array<Array<InstanceViewerParticipants>> {
        const result = [[], [], [], [], [], [], [], [], [], [], [], [], [], [], [], []];
        for (const participant of participants.filter(i_participant => i_participant.role === role))
            result[participant.hero_class_id].push(participant);
        return result.filter(classes => classes.length > 0);
    }

}
