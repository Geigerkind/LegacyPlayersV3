import {Component, OnInit} from "@angular/core";
import {DataService} from "../../../../../../../../service/data";
import {SelectOption} from "../../../../../../../../template/input/select_input/domain_value/select_option";
import {TAGS} from "../../../../../data/tags";

@Component({
    selector: "Edit",
    templateUrl: "./edit.html",
    styleUrls: ["./edit.scss"]
})
export class EditComponent implements OnInit {

    expansions: Array<SelectOption> = [];
    selected_expansion: number = 0;

    tags: Array<any> = [];
    selected_tags: Array<any> = [];

    constructor(
        private dataService: DataService
    ) {
        this.expansions = this.dataService.expansions;
        this.tags = TAGS.map((tag, index) => {
            return {id: index, label: tag};
        });
    }

    ngOnInit(): void {
    }

    save_paste(): void {

    }
}
