#ifndef _RPLL_PLAYER_HOOKS_H
#define _RPLL_PLAYER_HOOKS_H

#include "rpll_spell_auras_hooks.h"
#include "Player.h"
#include "Item.h"

class RPLLPlayerHooks {
public:
    static void SendNewItem(Unit* unit, Item *item, uint32_t count, bool received, bool created, bool broadcast, bool sendChatMessage);
    // Note: Use the player for the unit
    static void EnvironmentalDamage(Unit* unit, EnviromentalDamage type, uint32_t damage, uint32_t result);
    // Used to get state on login/logout instance enter etc.
    static void SetMap(Unit* unit);
};

#endif