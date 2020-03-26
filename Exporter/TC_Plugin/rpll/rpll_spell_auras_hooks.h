#ifndef _RPLL_SPELL_AURAS_HOOKS_H
#define _RPLL_SPELL_AURAS_HOOKS_H

#include "rpll_hooks.h"
#include "SpellAuras.h"

class RPLLSpellAurasHooks {
public:
    static void AuraCreate(Aura* result);
    // IMPORTANT: Save the amount here BEFORE calling the original function
    static void AuraSetStackAmount(Aura* aura, uint32_t oldAmount);
};

#endif