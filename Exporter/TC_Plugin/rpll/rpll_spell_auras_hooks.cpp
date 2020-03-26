#include "rpll_spell_auras_hooks.h"

void RPLLSpellAurasHooks::AuraCreate(Aura* result) {
    if (result == nullptr)
        return;
    auto owner = result->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    RPLLHooks::AuraApplication(result->GetCaster(), owner->ToUnit(), uint32_t(result->GetId()), uint32_t(result->GetStackAmount()), true);
}

void RPLLSpellAurasHooks::AuraSetStackAmount(Aura* aura, uint32_t oldAmount) {
    if (aura == nullptr)
        return;
    auto owner = aura->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    
    auto amount = uint32_t(aura->GetStackAmount());
    if (amount != oldAmount)
        RPLLHooks::AuraApplication(aura->GetCaster(), owner->ToUnit(), uint32_t(aura->GetId()), uint32_t(aura->GetStackAmount()), amount > oldAmount);
}