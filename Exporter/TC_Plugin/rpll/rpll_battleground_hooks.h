#ifndef _RPLL_BG_HOOKS_H
#define _RPLL_BG_HOOKS_H

#include "rpll_hooks.h"
#include "Battlegrounds/Battleground.h"

class RPLLBattlegroundHooks {
public:
    static void StartBattleground(Battleground* battleground);
    static void EndBattleground(Battleground* battleground, uint32_t *scores);
};

#endif