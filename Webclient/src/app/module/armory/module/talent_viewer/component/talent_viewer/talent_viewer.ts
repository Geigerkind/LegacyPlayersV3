import {Component} from "@angular/core";
import {Localized} from "../../../../../../domain_value/localized";
import {HeroClass} from "../../../../../../domain_value/hero_class";
import {DataService} from "../../../../../../service/data";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {Talent} from "../../module/talent_tab/value_object/talent";

@Component({
    selector: "TalentViewer",
    templateUrl: "./talent_viewer.html",
    styleUrls: ["./talent_viewer.scss"]
})
export class TalentViewerComponent {

    expansions: Array<SelectOption> = [];
    hero_classes: Array<SelectOption> = [];

    private static talent_specs: Map<number, Map<number, Map<number, [string, Array<Array<Talent>>]>>> = new Map([
        // Vanilla
        [1, new Map([
            // Warrior
            [1, new Map([
                [1, ["Arms", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [12282, 12663, 12664],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_ambush"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [16462, 16463, 16464, 16465, 16466],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_parry"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [12286, 12658, 12659],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_gouge"
                        },
                        {is_filler: true, row_index: 0, column_index: 3}
                    ]
                ]]],
                [2, ["Fury", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [12321, 12835, 12836, 12837, 12838],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_purge"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [12320, 12852, 12853, 12855, 12856],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_rogue_eviscerate"
                        },
                        {is_filler: true, row_index: 0, column_index: 3}
                    ]
                ]]],
                [3, ["Protection", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [12298, 12724, 12725, 12726, 12727],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_shield_06"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [12297, 12750, 12751, 12752, 12753],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_mirrorimage"
                        },
                        {is_filler: true, row_index: 0, column_index: 3}
                    ]
                ]]],
            ]),
            ]])]
    ]);

    private static points_per_expansion: Map<number, number> = new Map([
       [1, 51],
       [2, 61],
       [3, 71]
    ]);

    private points_spend_per_tab: [number, number, number] = [0, 0, 0];

    selected_expansion: number = 1;
    selected_hero_class: number = 1;

    constructor(
        private dataService: DataService
    ) {
        this.expansions = this.dataService.expansions;
        this.dataService.hero_classes.subscribe((hero_classes: Array<Localized<HeroClass>>) =>
            hero_classes.forEach(hero_class => this.hero_classes.push({
                value: hero_class.base.id,
                label_key: hero_class.localization
            })));
    }

    get points_spend(): number {
        return this.points_spend_per_tab[0] + this.points_spend_per_tab[1] + this.points_spend_per_tab[2];
    }

    getTabTree(tab_index: number): Array<Array<Talent>> {
        return TalentViewerComponent.talent_specs.get(this.selected_expansion).get(this.selected_hero_class).get(tab_index)[1];
    }

    getTabLabel(tab_index: number): string {
        return TalentViewerComponent.talent_specs.get(this.selected_expansion).get(this.selected_hero_class).get(tab_index)[0];
    }

    get pointsPerExpansion(): number {
        return TalentViewerComponent.points_per_expansion.get(this.selected_expansion);
    }

    pointsChanged(tab_index: number, amount: number): void {
        this.points_spend_per_tab[tab_index - 1] = amount;
    }

}
