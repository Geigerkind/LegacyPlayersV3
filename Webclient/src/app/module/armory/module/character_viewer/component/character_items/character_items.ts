import {Component, Input} from "@angular/core";

@Component({
    selector: "CharacterItems",
    templateUrl: "./character_items.html",
    styleUrls: ["./character_items.scss"]
})
export class CharacterItemsComponent {

    @Input() character: any;

}
