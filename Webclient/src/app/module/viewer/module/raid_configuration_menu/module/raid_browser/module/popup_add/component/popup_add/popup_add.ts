import {Component, EventEmitter, Input, OnChanges, Output} from "@angular/core";
import {Preset} from "../../../../domain_value/preset";
import {SelectOption} from "../../../../../../../../../../template/input/select_input/domain_value/select_option";
import {NotificationService} from "../../../../../../../../../../service/notification";
import {Severity} from "../../../../../../../../../../domain_value/severity";

@Component({
    selector: "PopupAdd",
    templateUrl: "./popup_add.html",
    styleUrls: ["./popup_add.scss"]
})
export class PopupAddComponent implements OnChanges {

    @Input() presets: Array<Preset>;
    @Output() close_widget: EventEmitter<void> = new EventEmitter();
    @Output() add_preset: EventEmitter<Preset> = new EventEmitter();

    selected_option: string = "";
    preset_name: string = "";
    selected_event_types: Array<any> = [];
    selected_sources: Array<any> = [];
    selected_targets: Array<any> = [];
    selected_abilities: Array<any> = [];
    selected_hit_types: Array<any> = [];

    constructor(
        private notificationService: NotificationService
    ) {
    }

    ngOnChanges(): void {
        if (this.presets.length > 0 && this.selected_option === "")
            this.selected_option = this.presets[0].name;
    }

    get select_options(): Array<SelectOption> {
        return this.presets.map(preset => {
            return {value: preset.name, label_key: preset.name};
        });
    }

    load_preset(): void {
        this.preset_name = this.selected_option;
    }

    save_preset(): void {
        if (this.preset_name.length < 1) {
            this.notificationService.propagate(Severity.Error, "Preset name must not be empty!");
            return;
        }

        this.add_preset.next({
            name: this.preset_name,
            event_types: this.selected_event_types.map(item => item.id),
            sources: this.selected_sources.map(item => item.id),
            targets: this.selected_targets.map(item => item.id),
            abilities: this.selected_abilities.map(item => item.id),
            hit_types: this.selected_hit_types.map(item => item.id),
        });
    }
}
