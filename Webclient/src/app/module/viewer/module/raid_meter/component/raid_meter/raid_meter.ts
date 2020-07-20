import {Component} from "@angular/core";

@Component({
    selector: "RaidMeter",
    templateUrl: "./raid_meter.html",
    styleUrls: ["./raid_meter.scss"]
})
export class RaidMeterComponent {

    some_time: number = 60;
    bars = [
        {
            hero_class_id: 1,
            name: "Peter Mafai",
            amount: 100000
        },
        {
            hero_class_id: 2,
            name: "Hans Peter",
            amount: 90000
        },
        {
            hero_class_id: 3,
            name: "Spongebob Squarepants",
            amount: 80000
        },
        {
            hero_class_id: 4,
            name: "Patrick Star",
            amount: 70000
        },
        {
            hero_class_id: 5,
            name: "Peter Parker",
            amount: 60000
        },
        {
            hero_class_id: 6,
            name: "Harry Potter",
            amount: 50000
        },
        {
            hero_class_id: 7,
            name: "Hermine Stranger",
            amount: 40000
        },
        {
            hero_class_id: 8,
            name: "Ragnar Lovebruk",
            amount: 30000
        },
        {
            hero_class_id: 9,
            name: "Linus Torvalds",
            amount: 20000
        },
        {
            hero_class_id: 10,
            name: "Im running out of names",
            amount: 10000
        },
        {
            hero_class_id: 11,
            name: "xxXxxHunterDamageGuyxXx",
            amount: 5000
        },
        {
            hero_class_id: 12,
            name: "Filthy Demon Hunter",
            amount: 1000
        }
    ];

    get_sorted_bars(): Array<any> {
        this.bars.sort((left, right) => right.amount - left.amount);
        return this.bars;
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


}
