#ifndef _RPLL_CM_HOOKS_H
#define _RPLL_CM_HOOKS_H

#include "rpll_hooks.h"

class RPLLCombatManagerHooks {
public:
    static void UpdateOwnerCombatState(Unit* unit, bool result);
};

#endif