#include "rpll_player_hooks.h"

void RPLLPlayerHooks::SendNewItem(const Unit *unit, const Item *item, const uint32_t count, const bool received, const bool created, const bool broadcast, const bool sendChatMessage)
{
    if (item == nullptr || unit == nullptr)
        return;

    if (!created && (broadcast || sendChatMessage) && count >= 1)
        RPLLHooks::Loot(unit, item->GetEntry(), count);
}

void RPLLPlayerHooks::EnvironmentalDamage(const Unit *unit, const EnviromentalDamage type, const uint32_t damage, const uint32_t result)
{
    if (result == 0)
        return;

    // CalcAbsorbResist has side effects, so we guess the absorb value here
    uint32_t absorb = 0;
    uint32_t resist = 0;
    if (type == DAMAGE_LAVA || type == DAMAGE_SLIME)
    {
        const uint32_t dmgCopy = damage;
        const DamageInfo dmgInfo(unit, unit, dmgCopy, nullptr, type == DAMAGE_LAVA ? SPELL_SCHOOL_MASK_FIRE : SPELL_SCHOOL_MASK_NATURE, DIRECT_DAMAGE, BASE_ATTACK);
        resist = Unit::CalcSpellResistedDamage(dmgInfo);
        absorb = damage - result - resist;
    }

    const RPLL_DamageSchool damageSchool = RPLL_DamageSchool::RPLL_PHYSICAL;
    switch (type)
    {
    case DAMAGE_LAVA:
    case DAMAGE_FIRE:
        damageSchool = RPLL_DamageSchool::RPLL_FIRE;
        break;
    case DAMAGE_SLIME:
        damageSchool = RPLL_DamageSchool::RPLL_NATURE;
        break;
    default:
        break;
    }
    RPLLHooks::DealMeleeDamage(unit, unit, RPLL_DamageHitType::RPLL_ENVIRONMENT, 0,
                               std::move(RPLLHooks::BuildRPLLDamage(damageSchool, result, resist, absorb)));
}

void RPLLPlayerHooks::SetMap(const Unit *unit)
{
    RPLLHooks::Position(unit, unit->GetPositionX(), unit->GetPositionY(), unit->GetPositionZ(), unit->GetOrientation());
    for (auto aura : unit->GetOwnedAuras())
        RPLLSpellAurasHooks::AuraCreate(aura.second);
    RPLLHooks::Summon(unit, unit->GetOwnerGUID().GetRawValue());
}