import {Component} from "@angular/core";
import {InstanceDataService} from "../../service/instance_data";
import {ActivatedRoute} from "@angular/router";
import {UnitService} from "../../service/unit";

@Component({
    selector: "Viewer",
    templateUrl: "./viewer.html",
    styleUrls: ["./viewer.scss"],
    providers: [
        InstanceDataService,
        UnitService
    ]
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
        });
    }

}
