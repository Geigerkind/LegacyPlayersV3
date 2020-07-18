#include "rpll_battleground_hooks.h"

void RPLLBattlegroundHooks::StartBattleground(Battleground* battleground) {
    if (battleground == nullptr)
        return;
    if (battleground->isBattleground()) {
        RPLLHooks::StartBattleground(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()));
    } else if (battleground->isRated()) {
        uint32_t teamId1 = battleground->GetArenaTeamIdForTeam(ALLIANCE);
        uint32_t teamId2 = battleground->GetArenaTeamIdForTeam(HORDE);
        RPLLHooks::StartRatedArena(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), teamId1, teamId2);
    } else {
        RPLLHooks::StartUnratedArena(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()));
    }
}

void RPLLBattlegroundHooks::EndBattleground(Battleground* battleground, uint32_t *scores) {
    if (battleground == nullptr || scores == nullptr)
        return;
    RPLL_PvP_Winner winner = RPLLHooks::mapPvPWinnerToRPLLPvPWinner(battleground->GetWinner());
    if (battleground->isBattleground()) {
        uint32_t scoreAlliance = scores[TEAM_ALLIANCE];
        uint32_t scoreHorde = scores[TEAM_HORDE];
        RPLLHooks::EndBattleground(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner, scoreAlliance, scoreHorde);
    } else if (battleground->isRated()) {
        uint32_t teamId1 = battleground->GetArenaTeamIdForTeam(ALLIANCE);
        uint32_t teamId2 = battleground->GetArenaTeamIdForTeam(HORDE);
        int32_t teamChange1 = battleground->GetArenaTeamRatingChangeForTeam(ALLIANCE);
        int32_t teamChange2 = battleground->GetArenaTeamRatingChangeForTeam(HORDE);
        RPLLHooks::EndRatedArena(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner, teamId1, teamId2, teamChange1, teamChange2);
    } else {
        RPLLHooks::EndUnratedArena(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner);
    }
}