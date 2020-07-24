import {Component, Input} from "@angular/core";

@Component({
    selector: "InstanceMap",
    templateUrl: "./instance_map.html",
    styleUrls: ["./instance_map.scss"]
})
export class InstanceMapComponent {
    @Input() map_icon: string;
    @Input() instance_label: string;
    @Input() html_href: string;
}
