import {Component} from "@angular/core";
import {InstanceDataService} from "../../service/instance_data";
import {ActivatedRoute} from "@angular/router";

@Component({
    selector: "Viewer",
    templateUrl: "./viewer.html",
    styleUrls: ["./viewer.scss"],
    providers: [InstanceDataService]
})
export class ViewerComponent {

    private instance_meta_id: number;

    constructor(
        private instanceDataService: InstanceDataService,
        private activatedRouteService: ActivatedRoute
    ) {
        this.activatedRouteService.paramMap.subscribe(params => {
            this.instance_meta_id = Number(params.get('instance_meta_id'));
            this.instanceDataService.instance_meta_id = this.instance_meta_id;

            this.instanceDataService.spell_casts.subscribe(events => {
                console.log("SpellCasts: ", events);
            });

            this.instanceDataService.deaths.subscribe(events => {
                console.log("Deaths: ", events);
            });

            this.instanceDataService.combat_states.subscribe(events => {
                console.log("Combat States: ", events);
            });

            this.instanceDataService.loot.subscribe(events => {
                console.log("Loot: ", events);
            });

            this.instanceDataService.positions.subscribe(events => {
                console.log("Positions: ", events);
            });

            this.instanceDataService.powers.subscribe(events => {
                console.log("Powers: ", events);
            });

            this.instanceDataService.aura_applications.subscribe(events => {
                console.log("Aura Applications: ", events);
            });

            this.instanceDataService.interrupts.subscribe(events => {
                console.log("Interrupts: ", events);
            });

            this.instanceDataService.spell_steals.subscribe(events => {
                console.log("Spell Steals: ", events);
            });

            this.instanceDataService.dispels.subscribe(events => {
                console.log("Dispels: ", events);
            });

            this.instanceDataService.threat_wipes.subscribe(events => {
                console.log("Threat Wipes: ", events);
            });

            this.instanceDataService.summons.subscribe(events => {
                console.log("Summons: ", events);
            });

            this.instanceDataService.melee_damage.subscribe(events => {
                console.log("Melee Damage: ", events);
            });

            this.instanceDataService.spell_damage.subscribe(events => {
                console.log("Spell Damage: ", events);
            });

            this.instanceDataService.heal.subscribe(events => {
                console.log("Heal: ", events);
            });

            this.instanceDataService.threat.subscribe(events => {
                console.log("Threat: ", events);
            });
        });
    }

}
