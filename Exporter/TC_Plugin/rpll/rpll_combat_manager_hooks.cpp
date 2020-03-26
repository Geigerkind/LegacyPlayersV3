#include "rpll_combat_manager_hooks.h"

void RPLLCombatManagerHooks::UpdateOwnerCombatState(Unit* unit, bool result) {
    if (!result || unit == nullptr)
        return;
    RPLLHooks::CombatState(unit, unit->IsInCombat());
    RPLLHooks::Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
    auto powerType = unit->GetPowerType();
    RPLLHooks::Power(unit, RPLLHooks::mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}