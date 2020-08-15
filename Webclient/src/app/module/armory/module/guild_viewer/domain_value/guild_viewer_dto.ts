import {GuildViewerMemberDto} from "./guild_viewer_member_dto";

export interface GuildViewerDto {
    guild_id: number;
    guild_name: string;
    ranks: Array<{ index: number; name: string; }>;
    member: Array<GuildViewerMemberDto>;
}
