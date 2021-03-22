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
                            icon: "ability_gouge",
                            points_to: { row_index: 2, column_index: 2}
                        },
                        {is_filler: true, row_index: 0, column_index: 3}
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [12285, 12697],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_warrior_charge"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [12295, 12676, 12677, 12678, 12679],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_enchantarmor",
                            points_to: { row_index: 2, column_index: 1}
                        },
                        {is_filler: true, row_index: 1, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [12287, 12665, 12666],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_thunderclap"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [12290, 12963],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_sword_05"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [12296],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_blessingofstamina"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [12834, 12849, 12867],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_backstab",
                            points_to: {row_index: 3, column_index: 2}
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [12163, 12711, 12712, 12713, 12714],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_axe_09"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [16493, 16494],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_searingarrow"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [12700, 12781, 12783, 12784, 12785],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_axe_06"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [12292],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_rogue_slicedice",
                            points_to: {row_index: 6, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [12284, 12701, 12702, 12703, 12704],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_mace_01"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [12281, 12812, 12813, 12814, 12815],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_sword_27"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 0,
                            spell_id: [12165, 12830, 12831, 12832, 12833],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_weapon_halbard_01"
                        },
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [12289, 12668, 23695],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_shockwave"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [12294],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_savageblow"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
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
                    ],
                    [
                        {is_filler: true, row_index: 1, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [12324, 12876, 12877, 12878, 12879],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_warcry"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [12322, 12999, 13000, 13001, 13002],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_stoneclawtotem"
                        },
                        {is_filler: true, row_index: 1, column_index: 3}
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [12329, 12950, 20496],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_warrior_cleave"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [12323],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_deathscream"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16487, 16489, 16492],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_summonimp"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [12318, 12857, 12858, 12860, 12861],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_battleshout"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [23584, 23585, 23586, 23587, 23588],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_dualwield"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [20502, 20503],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_sword_48"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [12317, 13045, 13046, 13047, 13048],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_unholyfrenzy",
                            points_to: {row_index: 5, column_index: 2}
                        },
                        {is_filler: true, row_index: 3, column_index: 3}
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [12862, 12330, 20497, 20498, 20499],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_decisivestrike"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [12328],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_deathpact",
                            points_to: {row_index: 6, column_index: 1}
                        },
                        {is_filler: true, row_index: 4, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [20504, 20505],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_rogue_sprint"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 0,
                            spell_id: [20500, 20501],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_ancestralguardian"
                        },
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [12319, 12971, 12972, 12973, 12974],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_ghoulfrenzy"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [23881],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_bloodlust"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
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
                            icon: "inv_shield_06",
                            points_to: {row_index: 2, column_index: 1}
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
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [12301, 12818],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_racial_bloodrage",
                            points_to: {row_index: 2, column_index: 0}
                        },
                        {is_filler: true, row_index: 1, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [12299, 12761, 12762, 12763, 12764],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_devotion"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [12300, 12959, 12960, 12961, 12962],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_magic_magearmor"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [12975],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_ashestoashes"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [12945, 12307, 12944],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_defend"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [12797, 12799, 12800],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_warrior_revenge"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [12303, 12788, 12789, 12791, 12792],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_innerrage"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [12308, 12810, 12811],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_warrior_sunder"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [12313, 12804, 12807],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_warrior_disarm"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [12302, 12765],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_reincarnation"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [12312, 12803],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_warrior_shieldwall"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [12809],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_thunderbolt",
                            points_to: {row_index: 6, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [12311, 12958],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_warrior_shieldbash"
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [16538, 16539, 16540, 16541, 16542],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_sword_20"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [23922],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_shield_05"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
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
