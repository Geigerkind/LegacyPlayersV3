import {GuildViewerMemberDto} from "./guild_viewer_member_dto";

export interface GuildViewerDto {
    guild_id: number;
    guild_name: string;
    member: Array<GuildViewerMemberDto>;
}
