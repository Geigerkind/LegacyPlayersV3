#include "rpll_unit_hooks.h"

void RPLLUnitHooks::SendAttackStateUpdate(CalcDamageInfo *damageInfo) {
    if (damageInfo == nullptr || damageInfo->Attacker == nullptr || damageInfo->Target == nullptr)
        return;
    RPLL_DamageHitType damageHitType = RPLLHooks::mapHitMaskToRPLLHitType(damageInfo->HitInfo);
    std::vector<RPLL_Damage> damages;
    damages.reserve(1);
    for (auto dmg : damageInfo->Damages) {
        if (dmg.Damage == 0 && dmg.Absorb == 0 && dmg.Resist == 0)
            continue;
        RPLL_DamageSchool damageSchool = RPLLHooks::mapSpellSchoolMaskToRPLLDamageSchool(dmg.DamageSchoolMask);
        damages.push_back(std::move(RPLLHooks::BuildRPLLDamage(damageSchool, uint32_t(dmg.Damage), uint32_t(dmg.Resist), uint32_t(dmg.Absorb))));
    }
    RPLLHooks::DealMeleeDamage(damageInfo->Attacker, damageInfo->Target, damageHitType, uint32_t(damageInfo->Blocked), std::move(damages));
}

void RPLLUnitHooks::SendSpellNonMeleeDamageLog(SpellNonMeleeDamage *damageInfo) {
    if (damageInfo == nullptr || damageInfo->attacker == nullptr || damageInfo->target == nullptr)
        return;

    RPLL_DamageSchool damageSchool = RPLLHooks::mapSpellSchoolMaskToRPLLDamageSchool(damageInfo->schoolMask);
    RPLLHooks::DealSpellDamage(damageInfo->attacker, damageInfo->target,
        uint32_t(damageInfo->blocked), uint32_t(damageInfo->SpellID),
        std::move(RPLLHooks::BuildRPLLDamage(damageSchool, uint32_t(damageInfo->damage), uint32_t(damageInfo->absorb), uint32_t(damageInfo->resist))));
}

void RPLLUnitHooks::DealHeal(HealInfo& healInfo) {
    RPLLHooks::Heal(healInfo.GetHealer(), healInfo.GetTarget(), healInfo.GetSpellInfo()->Id, healInfo.GetHeal(), healInfo.GetEffectiveHeal(), healInfo.GetAbsorb());
}

void RPLLUnitHooks::SendPeriodicAuraLog(SpellPeriodicAuraLogInfo* pInfo) {
    if (pInfo == nullptr)
        return;

    AuraEffect const* aura = pInfo->auraEff;
    Unit* caster = aura->GetCaster();
    std::vector<Unit*> targets;
    aura->GetTargetList(targets);
    switch (aura->GetAuraType()) {
        case SPELL_AURA_PERIODIC_DAMAGE:
        case SPELL_AURA_PERIODIC_DAMAGE_PERCENT:
            for (Unit* target : targets) {
                RPLL_DamageSchool damageSchool = RPLLHooks::mapSpellSchoolMaskToRPLLDamageSchool(uint32_t(aura->GetSpellInfo()->SchoolMask)); 
                RPLLHooks::DealSpellDamage(caster, target, 0, aura->GetId(),
                    std::move(RPLLHooks::BuildRPLLDamage(damageSchool, uint32_t(pInfo->damage), uint32_t(pInfo->absorb), uint32_t(pInfo->resist))));
            }
            return;
        case SPELL_AURA_PERIODIC_HEAL:
        case SPELL_AURA_OBS_MOD_HEALTH:
            // This is taken care of in DealHeal
            return;
        case SPELL_AURA_OBS_MOD_POWER:
        case SPELL_AURA_PERIODIC_ENERGIZE:
            // This is not interesting, since we track all health changes anyway
            // This may give us some procs though
            // Maybe Extra attacks?
            return;
        case SPELL_AURA_PERIODIC_MANA_LEECH:
            // Also not interesting, atleast yet
            // Maybe we can deal "damage" in form of any power type
            return;
        default:
            return;
    }
}

void RPLLUnitHooks::Kill(Unit* attacker, Unit* victim) {
    if (attacker != nullptr && victim != nullptr)
        RPLLHooks::Death(attacker, victim);
}

void RPLLUnitHooks::RemoveAurasDueToSpellByDispel(Unit* target, uint32_t spellId, uint32_t dispellerSpellId, ObjectGuid casterGUID, WorldObject* dispeller, uint8 chargesRemoved) {
    if (casterGUID.IsUnit() && dispeller->GetGUID().IsUnit() && dispeller->ToUnit() != nullptr)
        RPLLHooks::Dispel(dispeller->ToUnit(), target, casterGUID, dispellerSpellId, spellId, chargesRemoved);
}

void RPLLUnitHooks::RemoveAurasDueToSpellBySteal(Unit* target, uint32_t spellId, uint32_t stealSpellId, ObjectGuid casterGUID, WorldObject* stealer) {
    if (casterGUID.IsUnit() && stealer->GetGUID().IsUnit() && stealer->ToUnit() != nullptr)
        RPLLHooks::SpellSteal(stealer->ToUnit(), target, casterGUID, stealSpellId, spellId, 1);
}

void RPLLUnitHooks::UpdatePosition(Unit* unit, float x, float y, float z, float orientation) {
    if (unit == nullptr)
        return;
    RPLLHooks::Position(unit,x,y,z,orientation);
}

// Upon getting into PoV of the player the health is set.
// So while we may loose some precision, we get rid of a lot redundant events
void RPLLUnitHooks::SetHealth(Unit* unit, uint32_t oldVal) {
    if (unit == nullptr || oldVal == 0 || unit->GetHealth() == oldVal)
        return;
    RPLLHooks::Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
}

void RPLLUnitHooks::SetMaxHealth(Unit* unit, uint32_t oldVal) {
    if (unit == nullptr || oldVal == 0 || unit->GetMaxHealth() == oldVal)
        return;
    RPLLHooks::Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
}

void RPLLUnitHooks::SetPower(Unit* unit, Powers powerType, uint32_t oldVal) {
    if (unit == nullptr || oldVal == 0 || unit->GetPower(powerType) == oldVal)
        return;
    RPLLHooks::Power(unit, RPLLHooks::mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}

void RPLLUnitHooks::SetMaxPower(Unit* unit, Powers powerType, uint32_t oldVal) {
    if (unit == nullptr || oldVal == 0 || unit->GetMaxPower(powerType) == oldVal)
        return;
    RPLLHooks::Power(unit, RPLLHooks::mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}

void RPLLUnitHooks::RemoveOwnedAura(Aura* aura) {
    if (aura == nullptr)
        return;
    auto owner = aura->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    RPLLHooks::AuraApplication(aura->GetCaster(), owner->ToUnit(), uint32_t(aura->GetId()), 0, false);
}

void RPLLUnitHooks::SetOwnerGUID(Unit* unit, ObjectGuid owner) {
    if (unit == nullptr)
        return;
    if (!unit->GetGUID().IsUnit() || !owner.IsUnit())
        return;
    RPLLHooks::Summon(unit, owner.GetRawValue());
}