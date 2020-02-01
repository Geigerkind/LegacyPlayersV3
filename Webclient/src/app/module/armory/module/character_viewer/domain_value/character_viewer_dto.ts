import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";

export interface CharacterViewerDto {
    history_id: number;
    character_id: number;
    gender: boolean;
    faction: boolean;
    race_id: number;
    hero_class_id: number;
    level: number;
    name: string;
    guild: { guild_id: number, name: string, rank: string } | undefined;
    history: Array<SelectOption>;
    gear: {
        gear_id: number;
        head: { item_id: number, quality: number, icon: string } | undefined;
        neck: { item_id: number, quality: number, icon: string } | undefined;
        shoulder: { item_id: number, quality: number, icon: string } | undefined;
        back: { item_id: number, quality: number, icon: string } | undefined;
        chest: { item_id: number, quality: number, icon: string } | undefined;
        shirt: { item_id: number, quality: number, icon: string } | undefined;
        tabard: { item_id: number, quality: number, icon: string } | undefined;
        wrist: { item_id: number, quality: number, icon: string } | undefined;
        main_hand: { item_id: number, quality: number, icon: string } | undefined;
        off_hand: { item_id: number, quality: number, icon: string } | undefined;
        ternary_hand: { item_id: number, quality: number, icon: string } | undefined;
        glove: { item_id: number, quality: number, icon: string } | undefined;
        belt: { item_id: number, quality: number, icon: string } | undefined;
        leg: { item_id: number, quality: number, icon: string } | undefined;
        boot: { item_id: number, quality: number, icon: string } | undefined;
        ring1: { item_id: number, quality: number, icon: string } | undefined;
        ring2: { item_id: number, quality: number, icon: string } | undefined;
        trinket1: { item_id: number, quality: number, icon: string } | undefined;
        trinket2: { item_id: number, quality: number, icon: string } | undefined;
    }
}
