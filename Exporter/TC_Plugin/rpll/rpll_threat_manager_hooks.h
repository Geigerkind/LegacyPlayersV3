#ifndef _RPLL_THREAT_MANAGER_HOOKS_H
#define _RPLL_THREAT_MANAGER_HOOKS_H

#include "rpll_hooks.h"
#include "Creature.h"

class RPLLThreatManagerHooks {
public:
    // Notes:
    // Owner => _owner or GetOwner()
    // AmountBefore => GetThreat of the target called BEFORE calling the original AddThreat function
    // AmountAfter => GetThreat of the target called AFTER calling the original AddThreat function
    static void AddThreat(Unit* owner, Unit* target, SpellInfo const* spell, float amountBefore, float amountAfter);
    static void ScaleThreat(Unit* owner, Unit* target, float factor);
};

#endif