#ifndef _RPLL_UNIT_HOOKS_H
#define _RPLL_UNIT_HOOKS_H

#include "rpll_hooks.h"
#include "SpellAuraEffects.h"

class RPLLUnitHooks {
public:
    static void SendAttackStateUpdate(CalcDamageInfo *damageInfo);
    static void SendSpellNonMeleeDamageLog(SpellNonMeleeDamage *damageInfo);
    static void SendPeriodicAuraLog(SpellPeriodicAuraLogInfo* pInfo);
    // IMPORTANT: Add hook AFTER original function
    static void DealHeal(HealInfo& healInfo);
    static void Kill(Unit* attacker, Unit* victim);
    // Note: Target is the Unit on whom it is called, i.e. "this"
    static void RemoveAurasDueToSpellByDispel(Unit* target, uint32_t spellId, uint32_t dispellerSpellId, ObjectGuid casterGUID, WorldObject* dispeller, uint8 chargesRemoved = 1);
    // Note: The implementation may not give you the used spell to steal
    // Set the stealSpellId to 0 in this case
    // The target is again the Unit, i.e. "this"
    static void RemoveAurasDueToSpellBySteal(Unit* target, uint32_t spellId, uint32_t stealSpellId, ObjectGuid casterGUID, WorldObject* stealer);
    // Note: Only call if the position update was successful
    static void UpdatePosition(Unit* unit, float x, float y, float z, float orientation);
    // IMPORTANT: These have to be called AFTER the original function.
    // BUT save the old value before!
    static void SetHealth(Unit* unit, uint32_t oldVal);
    static void SetMaxHealth(Unit* unit, uint32_t oldVal);
    static void SetPower(Unit* unit, Powers powerType, uint32_t oldVal);
    static void SetMaxPower(Unit* unit, Powers powerType, uint32_t oldVal);
    // IMPORTANT: Make sure to call it BEFORE the original call
    static void RemoveOwnedAura(Aura* aura);
    // Note: Unit is "this"
    static void SetOwnerGUID(Unit* unit, ObjectGuid owner);
};

#endif