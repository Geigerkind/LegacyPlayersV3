import {Component} from "@angular/core";

@Component({
    selector: "ConsentTable",
    templateUrl: "./consent_table.html",
    styleUrls: ["./consent_table.scss"]
})
export class ConsentTableComponent {

    sampleCharacters: { hero_class_id: number, name: string, consent: boolean }[] = [
        { hero_class_id: 2, name: "Peter", consent: true },
        { hero_class_id: 3, name: "PeterPan", consent: true },
        { hero_class_id: 4, name: "PanPeter", consent: false }
    ];

}
