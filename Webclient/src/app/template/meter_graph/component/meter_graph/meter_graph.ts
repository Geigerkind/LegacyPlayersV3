import {Component, EventEmitter, Input, Output} from "@angular/core";
import {RaidMeterSubject} from "../../domain_value/raid_meter_subject";
import {of} from "rxjs";

@Component({
    selector: "MeterGraph",
    templateUrl: "./meter_graph.html",
    styleUrls: ["./meter_graph.scss"]
})
export class MeterGraphComponent {
    @Input() bar_subjects: Map<number, RaidMeterSubject> = new Map();
    @Input() bar_tooltips: Map<number, any> = new Map();
    @Input() bars: Array<[number, number]> = [];
    @Input() per_second_duration: number = 1;
    @Input() show_per_second: boolean = true;
    @Input() show_percent: boolean = true;

    @Output() bar_clicked: EventEmitter<[number, number]> = new EventEmitter();

    get_total(): number {
        return this.bars.reduce((acc, bar) => acc + bar[1], 0);
    }

    get_weighted_bar_fraction(amount: number): number {
        return amount / this.bars.reduce((acc, bar) => bar[1] > acc ? bar[1] : acc, 0);
    }

    get_fraction(amount: number): number {
        return amount / this.get_total();
    }

    get_dps(amount: number): number {
        return amount / this.per_second_duration;
    }

    get_bar_subject(subject_id: number): RaidMeterSubject {
        if (this.bar_subjects.has(subject_id))
            return this.bar_subjects.get(subject_id);
        return {
            id: subject_id,
            name: of("Unknown"),
            color_class: of("hero_class_bg_1"),
            icon: of("/assets/wow_icon/inv_misc_questionmark.jpg")
        };
    }

    get_width_style(amount: number): string {
        const bar_fraction = this.get_weighted_bar_fraction(amount);
        if (!this.show_percent)
            return "calc(max(0px, calc(" + bar_fraction.toString() + " * calc(100% - 25px))))";
        return "calc(max(0px, calc(" + bar_fraction.toString() + " * calc(100% - 85px))))";
    }

    get_amount(amount: number): string {
        if (amount % 1 === 0)
            return amount.toString();
        return amount.toFixed(1);
    }
}
