#include "rpll_spell_hooks.h"

void RPLLSpellHooks::SendCastResult(const Spell* spell, SpellCastResult result) {
    if (spell == nullptr || result != SpellCastResult::SPELL_FAILED_INTERRUPTED)
        return;

    auto caster = spell->GetCaster();
    if (!caster->GetGUID().IsUnit())
        return;
    RPLLHooks::Interrupt(caster->ToUnit(), spell->GetSpellInfo()->Id);
}

void RPLLSpellHooks::DoDamageAndTriggers(const Spell* spell, uint32 hitMask) {
    auto caster = spell->GetCaster();
    if (!caster->GetGUID().IsUnit())
        return;
    
    RPLL_DamageHitType damageHitType = RPLLHooks::mapHitMaskToRPLLHitType(hitMask);

    auto unitTarget = spell->m_targets.GetUnitTarget();
    if (unitTarget != nullptr) {
        RPLLHooks::SpellCast(caster->ToUnit(), unitTarget->GetGUID().GetRawValue(), spell->GetSpellInfo()->Id, damageHitType);
        return;
    }
    if (spell->m_targets.GetCorpseTarget() != nullptr
        || spell->m_targets.GetItemTarget() != nullptr
        || spell->m_targets.GetObjectTarget() != nullptr
        || spell->m_targets.GetGOTarget() != nullptr
    ) {
        RPLLHooks::SpellCast(caster->ToUnit(), 0, spell->GetSpellInfo()->Id, damageHitType);
        return;
    }

    // Assume self cast
    RPLLHooks::SpellCast(caster->ToUnit(), caster->GetGUID().GetRawValue(), spell->GetSpellInfo()->Id, damageHitType);
}