#include "rpll_spell_auras_hooks.h"

void RPLLSpellAurasHooks::AuraCreate(const Aura* result) {
    if (result == nullptr)
        return;
    const auto owner = result->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    RPLLHooks::AuraApplication(result->GetCaster(), owner->ToUnit(), uint32_t(result->GetId()), uint32_t(result->GetStackAmount()), true);
}

void RPLLSpellAurasHooks::AuraSetStackAmount(const Aura* aura, const uint32_t oldAmount) {
    if (aura == nullptr)
        return;
    const auto owner = aura->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    
    const auto amount = uint32_t(aura->GetStackAmount());
    if (amount != oldAmount)
        RPLLHooks::AuraApplication(aura->GetCaster(), owner->ToUnit(), uint32_t(aura->GetId()), uint32_t(aura->GetStackAmount()), amount > oldAmount);
}