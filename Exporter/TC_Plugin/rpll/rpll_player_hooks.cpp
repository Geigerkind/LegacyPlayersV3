#include "rpll_player_hooks.h"

void RPLLPlayerHooks::SendNewItem(Unit* unit, Item *item, uint32_t count, bool received, bool created, bool broadcast, bool sendChatMessage) {
    if (item == nullptr || unit == nullptr)
        return;
    
    if (!created && (broadcast || sendChatMessage) && count >= 1)
        RPLLHooks::Loot(unit, item->GetEntry(), count);
}

void RPLLPlayerHooks::EnvironmentalDamage(Unit* unit, EnviromentalDamage type, uint32_t damage, uint32_t result) {
    if (result == 0)
        return;

    // CalcAbsorbResist has side effects, so we guess the absorb value here
    uint32 absorb = 0;
    uint32 resist = 0;
    if (type == DAMAGE_LAVA || type == DAMAGE_SLIME) {
        uint32_t dmgCopy = damage;
        DamageInfo dmgInfo(unit, unit, dmgCopy, nullptr, type == DAMAGE_LAVA ? SPELL_SCHOOL_MASK_FIRE : SPELL_SCHOOL_MASK_NATURE, DIRECT_DAMAGE, BASE_ATTACK);
        resist = Unit::CalcSpellResistedDamage(dmgInfo);
        absorb = damage - result - resist;
    }

    RPLL_DamageSchool damageSchool = RPLL_DamageSchool::RPLL_PHYSICAL;
    switch (type) {
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

void RPLLPlayerHooks::SetMap(Unit* unit) {
    RPLLHooks::Position(unit,unit->GetPositionX(), unit->GetPositionY(), unit->GetPositionZ(), unit->GetOrientation());
    for (auto aura : unit->GetOwnedAuras())
        RPLLSpellAurasHooks::AuraCreate(aura.second);
    RPLLHooks::Summon(unit, unit->GetOwnerGUID().GetRawValue());
}