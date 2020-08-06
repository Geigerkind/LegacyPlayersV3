#include "rpll_combat_manager_hooks.h"

void RPLLCombatManagerHooks::UpdateOwnerCombatState(const Unit* unit, const bool result) {
    if (!result || unit == nullptr)
        return;
    RPLLHooks::CombatState(unit, unit->IsInCombat());
    RPLLHooks::Power(unit, RPLL_PowerType::RPLL_HEALTH, static_cast<uint32_t>(unit->GetMaxHealth()), static_cast<uint32_t>(unit->GetHealth()));
    const auto powerType = unit->GetPowerType();
    RPLLHooks::Power(unit, RPLLHooks::mapPowersToRPLLPowerType(powerType), static_cast<uint32_t>(unit->GetMaxPower(powerType)), static_cast<uint32_t>(unit->GetPower(powerType)));
}