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
    private static EVENT_TYPES: Array<[number, string]> = [
        [0, "Spell casts"],
        [1, "Deaths"],
        [2, "Combat states"],
        [3, "Loot"],
        [4, "Positions"],
        [5, "Power gained"],
        [6, "Aura applications"],
        [7, "Interrupts"],
        [8, "Spell steals"],
        [9, "Dispels"],
        [10, "Threat wipes"],
        [11, "Summons"],
        [12, "Melee damage"],
        [13, "Spell damage"],
        [14, "Heal"],
        [15, "Threat"]
    ];

    @Input() presets: Array<Preset>;
    @Output() close_widget: EventEmitter<void> = new EventEmitter();
    @Output() add_preset: EventEmitter<Preset> = new EventEmitter();

    load_presets: Array<SelectOption> = [];
    event_types: Array<any> = [];

    selected_preset: string = "";
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
        if (this.presets.length > 0 && this.selected_preset === "")
            this.selected_preset = this.presets[0].name;

        this.load_presets = this.select_presets;
        this.event_types = this.select_event_types;
    }

    get select_presets(): Array<SelectOption> {
        return this.presets.map(preset => {
            return {value: preset.name, label_key: preset.name};
        });
    }

    get select_event_types(): Array<any> {
        return [{id: -1, label: "Everything"}].concat(PopupAddComponent.EVENT_TYPES.map(evt_type => {
            return {id: evt_type[0], label: evt_type[1]};
        }));
    }

    load_preset(): void {
        this.preset_name = this.selected_preset;
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

    selected_event_types_changed(selected_items: Array<any>): void {
        const old_contains_everything: boolean = !!this.selected_event_types.find(item => item.id === -1);
        const new_contains_everything: boolean = !!selected_items.find(item => item.id === -1);
        if (!old_contains_everything && new_contains_everything) {
            this.selected_event_types = selected_items.filter(item => item.id === -1);
        } else if (old_contains_everything && selected_items.length > 1) {
            this.selected_event_types = selected_items.filter(item => item.id !== -1);
        } else {
            this.selected_event_types = selected_items;
        }
    }
}
