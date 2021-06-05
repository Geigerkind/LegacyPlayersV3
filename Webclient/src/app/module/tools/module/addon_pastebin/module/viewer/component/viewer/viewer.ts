import {Component, OnInit} from "@angular/core";
import {Paste} from "../../../../domain_value/paste";
import {Meta, Title} from "@angular/platform-browser";
import {TAGS} from "../../../../../data/tags";
import {NotificationService} from "../../../../../../../../service/notification";
import {Severity} from "../../../../../../../../domain_value/severity";
import {Clipboard} from "@angular/cdk/clipboard";

@Component({
    selector: "Viewer",
    templateUrl: "./viewer.html",
    styleUrls: ["./viewer.scss"],
})
export class ViewerComponent implements OnInit {

    paste: Paste = {
        id: 42,
        title: "TEST",
        expansion_id: 1,
        addon_name: "Weak Auras",
        tags: [1, 2, 3],
        description: "Some description",
        content: "WAMBO"
    };

    constructor(
        private metaService: Meta,
        private titleService: Title,
        private notificationService: NotificationService,
        private clipboard: Clipboard
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

    save_to_clipboard(): void {
        this.clipboard.copy(this.paste.content);
        this.notificationService.propagate(Severity.Success, "Paste saved to clipboard!");
    }

    get tags(): string {
        return this.paste.tags.map(tag => TAGS[tag]).join(", ");
    }
}
