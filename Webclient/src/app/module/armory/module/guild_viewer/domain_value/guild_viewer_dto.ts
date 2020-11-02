export interface GuildViewerDto {
    guild_id: number;
    guild_name: string;
    ranks: Array<{ index: number; name: string; }>;
}
