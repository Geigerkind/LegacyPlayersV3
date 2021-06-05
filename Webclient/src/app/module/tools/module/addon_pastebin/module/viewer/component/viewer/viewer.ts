import {Component, OnInit} from "@angular/core";
import {Paste} from "../../../../domain_value/paste";
import {Meta, Title} from "@angular/platform-browser";

@Component({
    selector: "Viewer",
    templateUrl: "./viewer.html",
    styleUrls: ["./viewer.scss"]
})
export class ViewerComponent implements OnInit {

    paste: Paste = {
        id: 42,
        title: "TEST",
        expansion_id: 1,
        addon_name: "Weak Auras",
        tags: [1,2,3],
        description: "Some description"
    };

    constructor(
        private metaService: Meta,
        private titleService: Title
    ) {
        this.titleService.setTitle("LegacyPlayers - Addon paste viewer");
        this.metaService.updateTag({
            name: "description",
            content: "Addon configuration pastes for Vanilla, TBC and WotLK. Pastes include for example WeakAuras, ElvUi Exports or simply lua configuration."
        });
    }

    ngOnInit(): void {
        this.titleService.setTitle(this.paste.title);
        this.metaService.updateTag({
            name: "description",
            content: this.paste.description
        });
    }


    get tags(): string {
        return "TODO";
    }
}
