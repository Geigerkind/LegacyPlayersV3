import {RankingCharacterMeta} from "./ranking_character_meta";

export interface RankingRow {
    character_id: number;
    character_meta: RankingCharacterMeta;
    amount: number | string;
}
