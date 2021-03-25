import {Component} from "@angular/core";
import {Localized} from "../../../../../../domain_value/localized";
import {HeroClass} from "../../../../../../domain_value/hero_class";
import {DataService} from "../../../../../../service/data";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {Talent} from "../../module/talent_tab/value_object/talent";
import {ActivatedRoute, Router} from "@angular/router";

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
                            icon: "spell_holy_blessingofstamina",
                            parent: {row_index: 1, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [12834, 12849, 12867],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_backstab",
                            parent: {row_index: 0, column_index: 2},
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
                            icon: "ability_searingarrow",
                            parent: {row_index: 2, column_index: 2}
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
                            icon: "ability_warrior_savageblow",
                            parent: {row_index: 4, column_index: 1}
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
                            icon: "ability_ghoulfrenzy",
                            parent: {row_index: 3, column_index: 2}
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
                            icon: "spell_nature_bloodlust",
                            parent: {row_index: 4, column_index: 1}
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
                            icon: "spell_holy_ashestoashes",
                            parent: {row_index: 1, column_index: 0}
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [12945, 12307, 12944],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_defend",
                            parent: {row_index: 0, column_index: 1}
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
                            icon: "inv_shield_05",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Paladin
            [2, new Map([
                [1, ["Holy", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [20262, 20263, 20264, 20265, 20266],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_golemthunderclap"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [20257, 20258, 20259, 20260, 20261],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_sleep"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 1, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [20205, 20206, 20207, 20209, 20208],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_arcane_blink"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [20224, 20225, 20330, 20331, 20332],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_thunderbolt"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [20237, 20238, 20239],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_holybolt"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [26573],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_innerfire"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [20234, 20235],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_layonhands"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [9453, 25836],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_unyieldingfaith"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [20210, 20212, 20213, 20214, 20215],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_greaterheal",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [20244, 20245],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_sealofwisdom"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [20216],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_heal",
                            parent: {row_index: 3, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [20359, 20360, 20361],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_healingaura"
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
                            spell_id: [5923, 5924, 5925, 5926, 25829],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_power"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [20473],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_searinglight",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Protection", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [20138, 20139, 20140, 20141, 20142],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_devotionaura"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [20127, 20130, 20135, 20136, 20137],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_defend",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [20189, 20192, 20193],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_ambush"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [20174, 20175],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_sealofprotection"
                        },
                        {is_filler: true, row_index: 1, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [20143, 20144, 20145, 20146, 20147],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_devotion"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [20217],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_magic_magearmor"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [20468, 20469, 20470],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_sealoffury"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [20148, 20149, 20150],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_shield_06",
                            parent: {row_index: 0, column_index: 2}
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [20096, 20097, 20098, 20099, 20100],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_magic_lesserinvisibilty"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [20487, 20488, 20489],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_sealofmight"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [20254, 20255, 20256],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_mindsooth"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [20911],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_lightningshield",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [20177, 20179, 20181, 20180, 20182],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_blessingofstrength"
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
                            spell_id: [20196, 20197, 20198, 20199, 20200],
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
                            spell_id: [20925],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_blessingofprotection",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Retribution", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [20042, 20045, 20046, 20047, 20048],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_fistofjustice"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [20101, 20102, 20103, 20104, 20105],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_windwalkon"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [25956, 25957],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_righteousfury"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [20335, 20336, 20337],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_holysmite"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [20060, 20061, 20062, 20063, 20064],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_parry"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [9452, 26016, 26021],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_vindication"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [20117, 20118, 20119, 20120, 20121],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_retributionaura",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [20375],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_innerrage"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [26022, 26023],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_persuitofjustice"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [9799, 25988],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_eyeforaneye"
                        },
                        {is_filler: true, row_index: 3, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [20091, 20092],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_auraoflight"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [20111, 20112, 20113],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_hammer_04"
                        },
                        {is_filler: true, row_index: 4, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [20218],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_mindvision"
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [20049, 20056, 20057, 20058, 20059],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_racial_avatar",
                            parent: {row_index: 2, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [20066],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_prayerofhealing"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]]
            ])],
            // Hunter
            [3, new Map([
                [1, ["Beast Mastery", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [19552, 19553, 19554, 19555, 19556],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_ravenform"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [19583, 19584, 19585, 19586, 19587],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_reincarnation"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [19557, 19558],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_eyeoftheowl"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [19549, 19550, 19551, 24386, 24387],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hunter_aspectofthemonkey"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [19609, 19610, 19612],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_misc_pelt_bear_03"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [24443, 19575],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_hunter_beastsoothe"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [19559, 19560],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_mount_jungletiger"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [19596],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_druid_dash"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [19616, 19617, 19618, 19619, 19620],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_bullrush"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [19572, 19573],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_hunter_mendpet"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [19598, 19599, 19600, 19601, 19602],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_misc_monsterclaw_04",
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [19578, 20895],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_druid_demoralizingroar"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [19577],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_devour",
                        },
                        {is_filler: true, row_index: 4, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [19590, 19592],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_abolishmagic"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [19621, 19622, 19623, 19624, 19625],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_misc_monsterclaw_03",
                            parent: {row_index: 3, column_index: 2}
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [19574],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_druid_ferociousbite",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Marksmanship", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [19407, 19412, 19413, 19414, 19415],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_stun"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [19416, 19417, 19418, 19419, 19420],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_wizardmark"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 1, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [19421, 19422, 19423, 19424, 19425],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hunter_snipershot"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [19426, 19427, 19429, 19430, 19431],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_searingarrow",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [19434],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_spear_07"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [19454, 19455, 19456, 19457, 19458],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_impalingbolt"
                        },
                        {is_filler: true, row_index: 2, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [19498, 19499, 19500],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_townwatch"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [19464, 19465, 19466, 19467, 19468],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hunter_quickshot"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [19485, 19487, 19488, 19489, 19490],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_piercedamage",
                            parent: {row_index: 1, column_index: 2}
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [19503],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_golemstormbolt"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [19461, 19462, 24691],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_upgrademoonglaive",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [19491, 19493, 19494],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_hunter_criticalshot"
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
                            spell_id: [19507, 19508, 19509, 19510, 19511],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_weapon_rifle_06"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [19506],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_trueshot",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Survival", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [24293, 24294, 24295],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_misc_head_dragon_black"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [19151, 19152, 19153],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_prayerofhealing"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [19295, 19297, 19298, 19301, 19300],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_parry"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [19184, 19387, 19388, 19389, 19390],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_stranglevines"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [19159, 19160],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_racial_bloodrage"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [19228, 19232, 19233, 19234, 19235],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_rogue_trip"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [19239, 19245],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_timestop"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [19255, 19256, 19257, 19258, 19259],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_twilight"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [19263],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_whirlwind",
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [19376, 19377],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_ensnare"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [19290, 19294, 24283],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_kick"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [19286, 19287],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_rogue_feigndeath"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [19370, 19371, 19373],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_blessingofstamina",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [19306],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_challange",
                            parent: {row_index: 2, column_index: 2}
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
                            spell_id: [19168, 19180, 19181, 24296, 24297],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_challange"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [19386],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_spear_02",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Rogue
            [4, new Map([
                [1, ["Assassination", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [14162, 14163, 14164],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_eviscerate"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [14144, 14148],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_fiegndead"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [14138, 14139, 14140, 14141, 14142],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_racial_bloodrage",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [14156, 14160, 14161],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_druid_disembowel"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [14158, 14159],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_deathscream"
                        },
                        {is_filler: true, row_index: 1, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [14165, 14166, 14167],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_slicedice"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [14179],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_decisivestrike"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [14168, 14169],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_warrior_riposte"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [14128, 14132, 14135, 14136, 14137],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_criticalstrike",
                            parent: {row_index: 0, column_index: 2}
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [16513, 16514, 16515, 16719, 16720],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_rogue_feigndeath"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [14113, 14114, 14115, 14116, 14117],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_poisons"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [14177],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_ice_lament",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [14174, 14175, 14176],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_kidneyshot"
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [14186, 14190, 14193, 14194, 14195],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_chilltouch",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [14983],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_earthbindtotem"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Combat", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [13741, 13793, 13792],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_gouge"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [13732, 13863],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_ritualofsacrifice"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [13712, 13788, 13789, 13790, 13791],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_invisibilty"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [13733, 13865, 13866],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_backstab"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [13713, 13853, 13854, 13855, 13856],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_parry",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [13705, 13832, 13843, 13844, 13845],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_marksmanship",
                            points_to: {row_index: 4, column_index: 2}
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [13742, 13872],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_shadowward"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [14251],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_challange",
                            parent: {row_index: 1, column_index: 1}
                        },
                        {is_filler: true, row_index: 2, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [13743, 13875],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_rogue_sprint"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [13754, 13867],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_kick"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [13706, 13804, 13805, 13806, 13807],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_weapon_shortblade_05"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [13715, 13848, 13849, 13851, 13852],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_dualwield",
                            parent: {row_index: 1, column_index: 2}
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [13709, 13800, 13801, 13802, 13803],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_mace_01"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [13877],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_warrior_punishingblow",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [13960, 13961, 13962, 13963, 13964],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_sword_27"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [13707, 13966, 13967, 13868, 13869],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_gauntlets_04"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [30919, 30920],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_blessingofstrength",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [18427, 18428, 18429],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_racial_avatar"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [13750],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_shadowworddominate"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Subtlety", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [13958, 13970, 13971, 13972, 13973],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_charm"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [14057, 14072, 14073, 14074, 14075],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_warrior_warcry"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [30892, 30893],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_rogue_feint"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [13981, 14066],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_magic_lesserinvisibilty"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [13975, 14062, 14063, 14064, 14065],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_magic_lesserinvisibilty"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [13976, 13979, 13980],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_fumble"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [14278],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_curse"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [14079, 14080, 14081],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_rogue_ambush"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [13983, 14070, 14071],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_mirrorimage"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [14076, 14094, 14095],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_sap"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [14171, 14172, 14173],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_sword_17",
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [30894, 30895],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_ambush"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [14185],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_antishadow",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [14082, 14083],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_summonsuccubus"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [16511],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_lifedrain",
                            parent: {row_index: 3, column_index: 2}
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [30902, 30903, 30904, 30905, 30906],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_lifedrain"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [14183],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_possession",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Priest
            [5, new Map([
                [1, ["Discipline", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [14522, 14788, 14789, 14790, 14791],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_magic_magearmor"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [14524, 14525, 14526, 14527, 14528],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_wand_01"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [14523, 14784, 14785, 14786, 14787],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_manaregentotem"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [14749, 14767],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_wordfortitude"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [14748, 14768, 14769],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_powerwordshield"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [14531, 14774],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_tranquility"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 2, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [14751],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_frost_windwalkon"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [14521, 14776, 14777],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_sleep",
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [14747, 14770, 14771],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_innerfire"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [14520, 14780, 14781, 14782, 14783],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hibernation"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [14750, 14772],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_manaburn"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [18551, 18552, 18553, 18554, 18555],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_enchantarmor",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [14752],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_divinespirit",
                            parent: {row_index: 2, column_index: 2}
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
                            spell_id: [18544, 18547, 18548, 18549, 18550],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_slowingtotem"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [10060],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_powerinfusion",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Holy", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [14913, 15012],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_healingfocus"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [14908, 15020, 17191],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_renew"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [14889, 15008, 15009, 15010, 15011],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_sealofsalvation"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 1, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [27900, 27901, 27902, 27903, 27904],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_spellwarding"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [18530, 18531, 18533, 18534, 18535],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_sealofwrath",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [15237],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_holynova",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [27811, 27815, 27816],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_blessedrecovery",
                        },
                        {is_filler: true, row_index: 2, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [14892, 15362, 15363],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_layonhands",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [27789, 27790],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_purify",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [14912, 15013, 15014],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_holy_heal02",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [14909, 15017],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_searinglightpriest",
                            parent: {row_index: 1, column_index: 2}
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [14911, 15018],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_prayerofhealing02",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [20711],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_enchant_essenceeternallarge",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [14901, 15028, 15029, 15030, 15031],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_spiritualguidence",
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
                            spell_id: [14898, 15349, 15354, 15355, 15356],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_moonglow",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [724],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_summonlightwell",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Shadow", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [15270, 15335, 15336, 15337, 15338],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_requiem",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [15268, 15323, 15324, 15325, 15326],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_gathershadows",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [15318, 15272, 15320],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_shadowward",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [15275, 15317],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_shadowwordpain",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [15260, 15327, 15328, 15329, 15330],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_burningspirit",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [15392, 15448],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_psychicscream",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [15273, 15312, 15313, 15314, 15316],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_unholyfrenzy",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [15407],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_siphonmana",
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [15274, 15311],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_magic_lesserinvisibilty",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [17322, 17323, 17325],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_chilltouch",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [15257, 15331, 15332, 15333, 15334],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_blackplague",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [15487],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_impphaseshift",
                            parent: {row_index: 2, column_index: 0}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [15286],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_unsummonbuilding",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [27839, 27840],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_improvedvampiricembrace",
                            parent: {row_index: 4, column_index: 1}
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
                            spell_id: [15259, 15307, 15308, 15309, 15310],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_twilight",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [15473],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_shadowform",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Shaman
            [7, new Map([
                [1, ["Elemental", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [16039, 16109, 16110, 16111, 16112],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_wispsplode"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [16035, 16105, 16106, 16107, 16108],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_fireball"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [16043, 16130],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_stoneclawtotem"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [28996, 28997, 28998],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_spiritarmor"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [16038, 16160, 16161],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_immolation"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [16164],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_manaburn"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [16040, 16113, 16114, 16115, 16116],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_frostward"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16041, 16117, 16118, 16119, 16120],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_callstorm"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [16086, 16544],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_sealoffire"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [29062, 29064, 29065],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_eyeofthestorm"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [30160, 29179, 29180],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_elementaldevastation"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [28999, 28900],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_stormreach"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [16089],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_fire_volcano"
                        },
                        {is_filler: true, row_index: 4, column_index: 2},
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [16578, 16579, 16580, 16581, 16582],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_lightning_lightningbolt01",
                            parent: {row_index: 2, column_index: 2}
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [16166],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_wispheal",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Enhancement", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [17485, 17486, 17487, 17488, 17489],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_grimward",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [16253, 16298, 16299, 16300, 16301],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_shield_06",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [16258, 16293],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_stoneskintotem",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [16255, 16302, 16303, 16304, 16305],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_thunderbolt",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [16262, 16287],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_spiritwolf",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [16261, 16290, 16291],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_lightningshield",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [16259, 16295],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_earthbindtotem",
                        },
                        {is_filler: true, row_index: 2, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16269],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_axe_10",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [16254, 16271, 16272, 16273, 16274],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_mirrorimage",
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [16256, 16281, 16282, 16283, 16284],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_ghoulfrenzy",
                            parent: {row_index: 1, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [16252, 16306, 16307, 16308, 16309],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_devotion",
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [29192, 29193],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_enchantweapon",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [16266, 29079, 29080],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_flametounge",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [16268],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_parry",
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
                            spell_id: [29082, 29084, 29086, 29087, 29088],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hunter_swiftstrike",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [17364],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_sealofmight",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Restoration", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [16182, 16226, 16227, 16228, 16229],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_magicimmunity",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [16179, 16214, 16215, 16216, 16217],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_manarecharge",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [16184, 16209],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_reincarnation",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [16176, 16235, 16240],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_undyingstrength",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [16173, 16222, 16223, 16224, 16225],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_moonglow",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [16180, 16196, 16198],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_stun",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [16181, 16230, 16232, 16233, 16234],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_healingwavelesser",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16189],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_nullward",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [29187, 29189, 29191],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_healingtouch",
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [16187, 16205, 16206, 16207, 16208],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_manaregentotem",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [16194, 16218, 16219, 16220, 16221],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_tranquility",
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [29206, 29205, 29202],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_healingway",
                        },
                        {is_filler: true, row_index: 4, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [16188],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_ravenform",
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
                            spell_id: [16178, 16210, 16211, 16212, 16213],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_wizardmark",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [16190],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_frost_summonwaterelemental",
                            parent: {row_index: 3, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Mage
            [8, new Map([
                [1, ["Arcane", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [11210, 12592],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_holy_dispelmagic"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [11222, 12839, 12840, 12841, 12842],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_devotion"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [11237, 12463, 12464, 16769, 16770],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_starfall"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [6057, 6085],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_wand_01"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [29441, 29444, 29445, 29446, 29447],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_astralrecalgroup"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [11213, 12574, 12575, 12576, 12577],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_manaburn"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [11247, 12606],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_abolishmagic"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [11242, 12467, 12469],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_wispsplode"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [28574],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_arcane_arcaneresilience"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [11252, 12605],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_detectlesserinvisibility"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [11255, 12598],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_frost_iceshock"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [18462, 18463, 18464],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_siphonmana"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [12043],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_enchantarmor"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [11232, 12500, 12501, 12502, 12503],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_charm",
                            parent: {row_index: 2, column_index: 2}
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [15058, 15059, 15060],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_teleport",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [12042],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_lightning",
                            parent: {row_index: 5, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Fire", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [11069, 12338, 12339, 12340, 12341],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_flamebolt",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [11103, 12357, 12358, 12359, 12360],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_meteorstorm",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [11119, 11120, 12846, 12847, 12848],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_incinerate",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [11100, 12353],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_flare",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [11078, 11080, 12342],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_fireball",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [18459, 18460],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_flameshock",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [11108, 12349, 12350],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_selfdestruct",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [11366],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_fire_fireball02",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [11083, 12351],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_fire",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [11095, 12872, 12873],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_soulburn",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [11094, 13043],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_firearmor",
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [29074, 29075, 29076],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_fire_masterofelements",
                        },
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [11115, 11367, 11368],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_wispheal",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [11113],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_holy_excorcism_02",
                            parent: {row_index: 2, column_index: 2}
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
                            spell_id: [11124, 12378, 12398, 12399, 12400],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_immolation",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [11129],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_fire_sealoffire",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Frost", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [11189, 28332],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_frost_frostward",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [11070, 12473, 16763, 16765, 16766],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_frostbolt02",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [29438, 29439, 29440],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_ice_magicdamage",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [11207, 12672, 15047, 15052, 15053],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_iceshard",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [11071, 12496, 12497],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_frostarmor",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [11165, 12475],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_frost_freezingbreath",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [11175, 12569, 12571],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_wisp",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [11151, 12952, 12953],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_frostbolt",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [12472],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_frost_wizardmark",
                        },
                        {is_filler: true, row_index: 2, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [11185, 12487, 12488],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_icestorm",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [16757, 16758],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_darkritual",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [11160, 12518, 12519],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_stun",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [11170, 12982, 12983, 12984, 12985],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_frostshock",
                            parent: {row_index: 1, column_index: 2}
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [11958],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_frost_frost",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [11190, 12489, 12490],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_glacier",
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
                            spell_id: [11180, 28592, 28593, 28594, 28595],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_frost_chillingblast",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [11426],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_ice_lament",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Warlock
            [9, new Map([
                [1, ["Afflication", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [18174, 18175, 18176, 18177, 18178],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_unsummonbuilding"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [17810, 17811, 17812, 17813, 17814],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_abominationexplosion"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [18179, 18180, 18181],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_curseofmannoroth"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [18213, 18372],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_haunting"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [18182, 18183],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_burningspirit"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [17804, 17805, 17806, 17807, 17808],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_lifedrain02"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [18827, 18829, 18830],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_curseofsargeras"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [17783, 17784, 17785, 17786, 17787],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_fingerofdeath"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [18288],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_contagion"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [18218, 18219],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_callofbone"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [18094, 18095],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_twilight"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [17864, 18393],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_siphonmana"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [18265],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_requiem"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [18223],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_grimward",
                            parent: {row_index: 2, column_index: 2}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [18310, 18311, 18312, 18313],
                            max_points: 4,
                            points_spend: 0,
                            icon: "spell_shadow_grimward",
                            parent: {row_index: 4, column_index: 2}
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [18271, 18272, 18273, 18274, 18275],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_shadetruesight",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [18220],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_darkritual",
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Demonology", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [18692, 18693],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_stone_04",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [18694, 18695, 18696],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_summonimp",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [18697, 18698, 18699, 18700, 18701],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_metamorphosis",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [18703, 18704],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_lifedrain",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [18705, 18706, 18707],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_summonvoidwalker",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [18731, 18743, 18744, 18745, 18746],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_magicalsentry",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [18754, 18755, 18756],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_shadow_summonsuccubus",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [18708],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_removecurse",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [18748, 18749, 18750, 18751, 18752],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_antishadow",
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [18709, 18710],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_impphaseshift",
                            parent: {row_index: 2, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [18769, 18770, 18771, 18772, 18773],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_shadowworddominate",
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [18821, 18822, 18823, 18824, 18825],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_enslavedemon",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [18788],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_psychicscream",
                        },
                        {is_filler: true, row_index: 4, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [18767, 18768],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_ammo_firetar",
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [23785, 23822, 23823, 23824, 23825],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_shadowpact",
                            parent: {row_index: 3, column_index: 2}
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [19028],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_gathershadows",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 2,
                            spell_id: [18774, 18775],
                            max_points: 2,
                            points_spend: 0,
                            icon: "inv_misc_gem_sapphire_01",
                        },
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Destruction", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [17793, 17796, 17801, 17802, 17803],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_shadowbolt",
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [17778, 17779, 17780, 17781, 17782],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_windsofwoe",
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 1, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [17788, 17789, 17790, 17791, 17792],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_shadow_deathpact",
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [18119, 18120, 18121, 18122, 18123],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_fire",
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [18126, 18127],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_firebolt",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [18128, 18129],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_curse",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [18130, 18131, 18132, 18133, 18134],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_flameshock",
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [17877],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_scourgebuild",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [18135, 18136],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_lavaspawn",
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [17917, 17918],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_corpseexplode",
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [17927, 17929, 17930, 17931, 17932],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_soulburn",
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [18096, 18073],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_fire_volcano",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [17815, 17833, 17834, 17835, 17836],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_immolation",
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [17959],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_shadow_shadowwordpain",
                            parent: {row_index: 2, column_index: 2}
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
                            spell_id: [17954, 17955, 17956, 17957, 17958],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_fire_immolation",
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [17962],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_fire_fireball",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])],
            // Druid
            [11, new Map([
                [1, ["Balance", [
                    [
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 0,
                            spell_id: [16814, 16815, 16816, 16817, 16818],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_abolishmagic"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [16689],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_natureswrath"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [17245, 17247, 17248, 17249],
                            max_points: 4,
                            points_spend: 0,
                            icon: "spell_nature_natureswrath",
                            parent: {row_index: 0, column_index: 1}
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [16918, 16919, 16920],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_stranglevines"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [16821, 16822, 16823, 16824, 16825],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_starfall"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [16902, 16903, 16904, 16905, 16906],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_staff_01"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 3,
                            spell_id: [16833, 16834, 16835],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_wispsplode"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [16836, 16839, 16840],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_thorns"
                        },
                        {is_filler: true, row_index: 2, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16864],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_crystalball"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [16819, 16820],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_naturetouchgrow"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [16909, 16910, 16911, 16912, 16913],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_purge",
                            parent: {row_index: 1, column_index: 1}
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [16850, 16923, 16924, 16925, 16926],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_arcane_starfire"
                        },
                        {is_filler: true, row_index: 3, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 4, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 1,
                            spell_id: [16880],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_naturesblessing"
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [16845, 16846, 16847],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_sentinal"
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [16896, 16897, 16899, 16900, 16901],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_moonglow",
                            parent: {row_index: 4, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [24858],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_forceofnature"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [2, ["Feral Combat", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [16934, 16935, 16936, 16937, 16938],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_hunter_pet_hyena"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [16858, 16859, 16860, 16861, 16862],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_druid_demoralizingroar"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [16947, 16948, 16949, 16950, 16951],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_ambush"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [16940, 16941],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_druid_bash"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [16929, 16930, 16931, 16932, 16933],
                            max_points: 5,
                            points_spend: 0,
                            icon: "inv_misc_pelt_bear_03"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 0,
                            spell_id: [17002, 24866],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_spiritwolf"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [16979],
                            max_points: 1,
                            points_spend: 0,
                            icon: "ability_hunter_pet_bear"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [16942, 16943, 16944],
                            max_points: 3,
                            points_spend: 0,
                            icon: "inv_misc_monsterclaw_04"
                        },
                        {is_filler: true, row_index: 2, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 0,
                            spell_id: [16966, 16968],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_shadow_vampiricaura"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [16972, 16974, 16975],
                            max_points: 3,
                            points_spend: 0,
                            icon: "ability_hunter_pet_cat"
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 2,
                            spell_id: [16952, 16954],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_ghoulfrenzy",
                            parent: {row_index: 2, column_index: 2}
                        },
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [16958, 16961],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_racial_cannibalize",
                            parent: {row_index: 2, column_index: 2}
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [16998, 16999],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_druid_ravage"
                        },
                        {is_filler: true, row_index: 4, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [16857],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_faeriefire"
                        },
                        {is_filler: true, row_index: 4, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 1,
                            spell_id: [17003, 17004, 17005, 17006, 24894],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_blessingofagility",
                            parent: {row_index: 3, column_index: 1}
                        },
                        {is_filler: true, row_index: 5, column_index: 2},
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [17007],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_unyeildingstamina"
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
                [3, ["Restoration", [
                    [
                        {is_filler: true, row_index: 0, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 1,
                            spell_id: [17050, 17051, 17053, 17054, 17055],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_regeneration"
                        },
                        {
                            is_filler: false,
                            row_index: 0,
                            column_index: 2,
                            spell_id: [17056, 17058, 17059, 17060, 17061],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_blessingofstamina"
                        },
                        {is_filler: true, row_index: 0, column_index: 3},
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 0,
                            spell_id: [17069, 17070, 17071, 17072, 17073],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_healingtouch"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 1,
                            spell_id: [17063, 17065, 17066, 17067, 17068],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_healingwavegreater"
                        },
                        {
                            is_filler: false,
                            row_index: 1,
                            column_index: 2,
                            spell_id: [17079, 17082],
                            max_points: 2,
                            points_spend: 0,
                            icon: "ability_druid_enrage"
                        },
                        {is_filler: true, row_index: 1, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 2, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 1,
                            spell_id: [17106, 17107, 17108],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_frost_windwalkon"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 2,
                            spell_id: [5570],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_insectswarm"
                        },
                        {
                            is_filler: false,
                            row_index: 2,
                            column_index: 3,
                            spell_id: [17118, 17119, 17120, 17121, 17122],
                            max_points: 5,
                            points_spend: 0,
                            icon: "ability_eyeoftheowl"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 3, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 1,
                            spell_id: [24968, 24969, 24970, 24971, 24972],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_holy_elunesgrace"
                        },
                        {is_filler: true, row_index: 3, column_index: 2},
                        {
                            is_filler: false,
                            row_index: 3,
                            column_index: 3,
                            spell_id: [17111, 17112, 17113],
                            max_points: 3,
                            points_spend: 0,
                            icon: "spell_nature_rejuvenation"
                        },
                    ],
                    [
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 0,
                            spell_id: [17116],
                            max_points: 1,
                            points_spend: 0,
                            icon: "spell_nature_ravenform",
                            parent: {row_index: 1, column_index: 0}
                        },
                        {is_filler: true, row_index: 4, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 2,
                            spell_id: [17104, 24943, 24944, 24945, 24946],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_protectionformnature",
                            parent: {row_index: 2, column_index: 2}
                        },
                        {
                            is_filler: false,
                            row_index: 4,
                            column_index: 3,
                            spell_id: [17123, 17124],
                            max_points: 2,
                            points_spend: 0,
                            icon: "spell_nature_tranquility"
                        },
                    ],
                    [
                        {is_filler: true, row_index: 5, column_index: 0},
                        {is_filler: true, row_index: 5, column_index: 1},
                        {
                            is_filler: false,
                            row_index: 5,
                            column_index: 2,
                            spell_id: [17074, 17075, 17076, 17077, 17078],
                            max_points: 5,
                            points_spend: 0,
                            icon: "spell_nature_tranquility"
                        },
                        {is_filler: true, row_index: 5, column_index: 3},
                    ],
                    [
                        {is_filler: true, row_index: 6, column_index: 0},
                        {
                            is_filler: false,
                            row_index: 6,
                            column_index: 1,
                            spell_id: [18562],
                            max_points: 1,
                            points_spend: 0,
                            icon: "inv_relics_idolofrejuvenation",
                            parent: {row_index: 3, column_index: 1}
                        },
                        {is_filler: true, row_index: 6, column_index: 2},
                        {is_filler: true, row_index: 6, column_index: 3},
                    ]
                ]]],
            ])]
        ])]
    ]);

    private static points_per_expansion: Map<number, number> = new Map([
        [1, 51],
        [2, 61],
        [3, 71]
    ]);

    private static non_existing_combinations: Array<[number, number]> = [
        [1, 6],
        [2, 6],
        [1, 10],
        [2, 10],
        [3, 10],
        [1, 12],
        [2, 12],
        [3, 12],
    ];

    private points_spend_per_tab: [number, number, number] = [0, 0, 0];
    private pre_selection: [Array<number>, Array<number>, Array<number>] = [[0], [0], [0]];

    selected_expansion: number = 1;
    saved_expansion: number = 1;
    selected_hero_class: number = 1;
    saved_hero_class: number = 1;

    tree_tab1: Array<Array<Talent>> = [];
    tree_tab2: Array<Array<Talent>> = [];
    tree_tab3: Array<Array<Talent>> = [];

    constructor(
        private dataService: DataService,
        private activatedRoute: ActivatedRoute,
        private router: Router
    ) {
        this.expansions = this.dataService.expansions;
        this.dataService.hero_classes.subscribe((hero_classes: Array<Localized<HeroClass>>) =>
            hero_classes.forEach(hero_class => this.hero_classes.push({
                value: hero_class.base.id,
                label_key: hero_class.localization
            })));

        this.tree_tab1 = this.getTabTree(1);
        this.tree_tab2 = this.getTabTree(2);
        this.tree_tab3 = this.getTabTree(3);

        activatedRoute.paramMap.subscribe(params => {
            if (!params.has("expansion_id"))
                return;
            const spec_1 = params.get("spec_1");
            const spec_2 = params.get("spec_2");
            const spec_3 = params.get("spec_3");

            this.pre_selection = [[], [], []];
            for (let i = 0; i < spec_1.length; ++i)
                this.pre_selection[0].push(Number(spec_1[i]));

            for (let i = 0; i < spec_2.length; ++i)
                this.pre_selection[1].push(Number(spec_2[i]));

            for (let i = 0; i < spec_3.length; ++i)
                this.pre_selection[2].push(Number(spec_3[i]));

            this.saved_expansion = Number(params.get("expansion_id"));
            this.saved_hero_class = Number(params.get("hero_class_id"));

            this.selected_expansion = this.saved_expansion;
            this.selected_hero_class = this.saved_hero_class;

            this.tree_tab1 = this.getTabTree(1);
            this.tree_tab2 = this.getTabTree(2);
            this.tree_tab3 = this.getTabTree(3);
        });
    }

    get points_spend(): number {
        return this.points_spend_per_tab[0] + this.points_spend_per_tab[1] + this.points_spend_per_tab[2];
    }

    private getTabTree(tab_index: number): Array<Array<Talent>> {
        if (!this.combinationExists)
            return [];

        const tree = TalentViewerComponent.talent_specs.get(this.selected_expansion).get(this.selected_hero_class).get(tab_index)[1];
        // Initialize
        let pre_selection_index = 0;
        for (let i = 0; i < tree.length; ++i) {
            for (let j = 0; j < 4; ++j) {
                if (pre_selection_index >= this.pre_selection[tab_index - 1].length)
                    return tree;
                const talent = tree[i][j];
                if (!talent.is_filler) {
                    talent.points_spend = Math.min(this.pre_selection[tab_index - 1][pre_selection_index], talent.max_points);
                    ++pre_selection_index;
                }
            }
        }
        return tree;
    }

    getTabLabel(tab_index: number): string {
        return TalentViewerComponent.talent_specs.get(this.selected_expansion).get(this.selected_hero_class).get(tab_index)[0];
    }

    get pointsPerExpansion(): number {
        return TalentViewerComponent.points_per_expansion.get(this.selected_expansion);
    }

    expansionChanged(expansion: number): void {
        this.selected_expansion = expansion;
        if (expansion !== this.saved_expansion) {
            this.configurationChanged();
        }
    }

    heroClassChanged(hero_class_id: number): void {
        this.selected_hero_class = hero_class_id;
        if (hero_class_id !== this.saved_hero_class) {
            this.configurationChanged();
        }
    }

    talentTreeChanged(tab_index: number, tree: Array<Array<Talent>>): void {
        let points_spend = 0;
        let new_point_dist = [];
        for (let i = 0; i < tree.length; ++i)
            for (let j = 0; j < 4; ++j)
                if (!tree[i][j].is_filler) {
                    points_spend += tree[i][j].points_spend;
                    new_point_dist.push(tree[i][j].points_spend);
                }
        this.points_spend_per_tab[tab_index - 1] = points_spend;
        this.pre_selection[tab_index - 1] = new_point_dist;
        this.replaceUrl();
    }

    private configurationChanged(): void {
        this.pre_selection = [[0], [0], [0]];
        this.tree_tab1 = this.getTabTree(1);
        this.tree_tab2 = this.getTabTree(2);
        this.tree_tab3 = this.getTabTree(3);
        this.replaceUrl();
    }

    private replaceUrl(): void {
        this.router.navigate(["/armory/talents/" + this.selected_expansion.toString()
        + "/" + this.selected_hero_class.toString()
        + "/" + this.pre_selection[0].map(x => x.toString()).join("")
        + "/" + this.pre_selection[1].map(x => x.toString()).join("")
        + "/" + this.pre_selection[2].map(x => x.toString()).join("")], {replaceUrl: true})
    }

    get combinationExists(): boolean {
        return TalentViewerComponent.non_existing_combinations
            .find(([exp, hero_class]) => exp === this.selected_expansion
                && hero_class === this.selected_hero_class) === undefined;
    }

}
