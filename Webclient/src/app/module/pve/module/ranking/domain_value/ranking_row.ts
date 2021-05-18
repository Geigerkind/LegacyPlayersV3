import {RankingCharacterMeta} from "./ranking_character_meta";

export interface RankingRow {
    attempt_ids: Array<number>;
    instance_meta_ids: Array<number>;
    character_id: number;
    character_meta: RankingCharacterMeta;
    amount: number | string;
}
