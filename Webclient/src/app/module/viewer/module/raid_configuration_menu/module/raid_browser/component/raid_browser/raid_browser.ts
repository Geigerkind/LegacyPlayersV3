import {Component} from "@angular/core";
import {Preset} from "../../domain_value/preset";

@Component({
    selector: "RaidBrowser",
    templateUrl: "./raid_browser.html",
    styleUrls: ["./raid_browser.scss"]
})
export class RaidBrowserComponent {

    presets: Array<Preset> = [
        { name: "Test", event_types: [], sources: [], targets: [], abilities: [], hit_types: []}
    ];

    show_remove_widget: boolean = false;
    show_add_widget: boolean = false;

    toggleRemoveWidget(): void {
        this.show_remove_widget = !this.show_remove_widget;
    }

    toggleAddWidget(): void {
        this.show_add_widget = !this.show_add_widget;
    }

    close_remove_widget(): void {
        this.show_remove_widget = false;
    }

    close_add_widget(): void {
        this.show_add_widget = false;
    }

    remove_preset(preset_name: string): void {
        this.presets = this.presets.filter(preset => preset.name !== preset_name);
    }
}
