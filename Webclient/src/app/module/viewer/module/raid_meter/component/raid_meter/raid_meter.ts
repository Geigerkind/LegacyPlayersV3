import {Component} from "@angular/core";
import {DamageDoneService} from "../../service/damage_done";
import {RaidMeterRow} from "../../domain_value/raid_meter_row";

@Component({
    selector: "RaidMeter",
    templateUrl: "./raid_meter.html",
    styleUrls: ["./raid_meter.scss"],
    providers: [
        DamageDoneService
    ]
})
export class RaidMeterComponent {

    some_time: number = 60;
    bars: Array<RaidMeterRow> = [];

    constructor(
        private damageDoneService: DamageDoneService
    ) {
        this.damageDoneService.rows
            .subscribe(rows => this.bars = rows);
    }

    get_weighted_bar_fraction(amount: number): number {
        return amount / this.bars.reduce((acc, bar) => bar.amount > acc ? bar.amount : acc, 0);
    }

    get_total(): number {
        return this.bars.reduce((acc, bar) => acc + bar.amount, 0);
    }

    get_fraction(amount: number): number {
        return amount / this.get_total();
    }

    get_total_dps(): number {
        return this.get_total() / this.some_time;
    }

    get_dps(amount: number): number {
        return amount / this.some_time;
    }

    selection_changed(selection: number): void {
        switch (selection) {
            case 1:
                this.damageDoneService.reload();
                break;
            case 2:
                break;
        }
    }

}
