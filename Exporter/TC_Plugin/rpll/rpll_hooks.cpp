#include "rpll_hooks.h"
#include <iostream>
#include <iomanip>
#include <assert.h>
#include <chrono>
#include "SpellMgr.h"

uint8_t RPLLHooks::API_VERSION = 0;

// This is in MS
uint64_t RPLLHooks::RAID_UPDATE_TIMEOUT = 1500;
uint64_t RPLLHooks::ARENA_UPDATE_TIMEOUT = 250;
uint64_t RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT = 1000;

float RPLLHooks::UPDATE_POSITION_LEEWAY = 15.0f;
float RPLLHooks::UPDATE_POSITION_LEEWAY_ARENA = 8.0f;
float RPLLHooks::UPDATE_POSITION_LEEWAY_NPC = 4.0f;
std::unordered_map<uint64_t, double> RPLLHooks::LAST_UNIT_POSITION = {};
std::unordered_map<uint64_t, RPLL_LastUpdate> RPLLHooks::LAST_UPDATE = {};

/*
 * ZMQ Messaging
 */
void *RPLLHooks::zmqContext = nullptr;
void *RPLLHooks::zmqSocket = nullptr;
void *RPLLHooks::GetZmqSocket()
{
    if (RPLLHooks::zmqSocket == nullptr)
    {
        std::cout << "Creating ZMQ Socket" << std::endl;
        RPLLHooks::zmqContext = zmq_ctx_new();
        RPLLHooks::zmqSocket = zmq_socket(RPLLHooks::zmqContext, ZMQ_PUSH);
        // Address of the Backend docker container
        int rc = zmq_connect(RPLLHooks::zmqSocket, "tcp://172.32.128.5:5690");
        assert(rc == 0);
    }
    return RPLLHooks::zmqSocket;
}

void RPLLHooks::SendZmqMessage(const ByteBuffer msg)
{
    zmq_send(GetZmqSocket(), msg.contents(), msg.size(), ZMQ_DONTWAIT);
}

/*
 * Time
 */
uint8_t RPLLHooks::GetCurrentTimeSize()
{
    return sizeof(uint64_t);
}
uint64_t RPLLHooks::GetCurrentTime()
{
    return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count());
}

/*
 * Conditions
 */
std::set<uint32_t> RPLLHooks::raidMapIds = {
    249, 309, 409, 469, 509, 531,                     // Vanilla
    532, 533, 534, 544, 548, 550, 564, 565, 568, 580, // TBC
    603, 615, 616, 624, 631, 649, 724                 // WOTLK
};
bool RPLLHooks::IsInRaid(const Unit *unit)
{
    return RPLLHooks::raidMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::raidMapIds.end();
}

std::set<uint32_t> RPLLHooks::arenaMapIds = {
    559, 562, 572, 617, 618};
bool RPLLHooks::IsInArena(const Unit *unit)
{
    return RPLLHooks::arenaMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::arenaMapIds.end();
}

std::set<uint32_t> RPLLHooks::battlegroundMapIds = {
    30, 489, 529, 566, 607, 628};
bool RPLLHooks::IsInBattleground(const Unit *unit)
{
    return RPLLHooks::battlegroundMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::battlegroundMapIds.end();
}

bool RPLLHooks::IsInInstance(const Unit *unit)
{
    return IsInRaid(unit) || IsInArena(unit) || IsInBattleground(unit);
}

/*
 * Health/Power data
 */
uint8_t RPLLHooks::GetMapMetaDataSize()
{
    return 2 * sizeof(uint32_t) + sizeof(uint8_t);
}
void RPLLHooks::AppendMapMetaData(ByteBuffer &msg, const Unit *unit)
{
    Map *map = unit->GetMap();
    msg << static_cast<uint32_t>(map->GetId());
    msg << static_cast<uint32_t>(map->GetInstanceId());
    msg << static_cast<uint8_t>(map->GetDifficulty());
}

/*
 * Message Meta Data
 */
uint64_t RPLLHooks::MESSAGE_COUNT = 0;
uint8_t RPLLHooks::GetMessageMetaDataSize()
{
    return 3 * sizeof(uint8_t) + GetCurrentTimeSize() + 8;
}
void RPLLHooks::AppendMessageMetaData(ByteBuffer &msg, const RPLL_MessageType msgType, const uint8_t msgLength)
{
    msg << API_VERSION;
    msg << static_cast<uint8_t>(msgType);
    msg << msgLength;
    msg << GetCurrentTime();
    msg << RPLLHooks::MESSAGE_COUNT++;
}

/*
 * Helper
 */

RPLL_Damage RPLLHooks::BuildRPLLDamage(const RPLL_DamageSchool damageSchool, const uint32_t damage, const uint32_t resisted_or_glanced, const uint32_t absorbed)
{
    RPLL_Damage dmg;
    dmg.damageSchool = damageSchool;
    dmg.damage = damage;
    dmg.resisted_or_glanced = resisted_or_glanced;
    dmg.absorbed = absorbed;
    return std::move(dmg);
}

void RPLLHooks::AppendRPLLDamage(ByteBuffer &msg, const RPLL_Damage &damage)
{
    msg << static_cast<uint8_t>(damage.damageSchool);
    msg << damage.damage;
    msg << damage.resisted_or_glanced;
    msg << damage.absorbed;
}

/*
 * Methods used to optimize
 */
bool RPLLHooks::IsPowerWithinTimeout(const Unit *unit, const RPLL_PowerType power)
{
    const uint64_t timeout = IsInRaid(unit) ? RPLLHooks::RAID_UPDATE_TIMEOUT
                                            : (IsInArena(unit) ? RPLLHooks::ARENA_UPDATE_TIMEOUT
                                                               : (IsInBattleground(unit) ? RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT
                                                                                         : std::numeric_limits<uint64_t>::max()));
    const auto now = GetCurrentTime();
    const auto guid = unit->GetGUID().GetRawValue();
    auto res = RPLLHooks::LAST_UPDATE.find(guid);
    if (power == RPLL_PowerType::RPLL_HEALTH && (res == RPLLHooks::LAST_UPDATE.end() || (now - res->second.health) >= timeout))
    {
        res->second.health = now;
        return true;
    }
    else if (res == RPLLHooks::LAST_UPDATE.end() || (now - res->second.power) >= timeout)
    {
        res->second.power = now;
        return true;
    }
    return false;
}

bool RPLLHooks::IsPositionWithinTimeout(const Unit *unit)
{
    const uint64_t timeout = IsInRaid(unit) ? RPLLHooks::RAID_UPDATE_TIMEOUT
                                            : (IsInArena(unit) ? RPLLHooks::ARENA_UPDATE_TIMEOUT
                                                               : (IsInBattleground(unit) ? RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT
                                                                                         : std::numeric_limits<uint64_t>::max()));
    const auto now = GetCurrentTime();
    const auto guid = unit->GetGUID().GetRawValue();
    auto res = RPLLHooks::LAST_UPDATE.find(guid);
    if (res == RPLLHooks::LAST_UPDATE.end() || (now - res->second.position) >= timeout)
    {
        res->second.position = now;
        return true;
    }
    return false;
}

bool RPLLHooks::HasSignificantPositionChange(const Unit *unit, const float x, const float y, const float z, const float orientation)
{
    const uint64_t unitGuid = unit->GetGUID().GetRawValue();
    auto oldPos = RPLLHooks::LAST_UNIT_POSITION.find(unitGuid);
    const double sumPos = std::abs(x) + std::abs(y) + std::abs(z) + std::abs(orientation);
    const float position_leeway = IsInArena(unit) ? RPLLHooks::UPDATE_POSITION_LEEWAY_ARENA : (unit->GetGUID().IsPlayer() ? RPLLHooks::UPDATE_POSITION_LEEWAY : RPLLHooks::UPDATE_POSITION_LEEWAY_NPC);

    if (oldPos == RPLLHooks::LAST_UNIT_POSITION.end() || std::fabs(oldPos->second - sumPos) >= position_leeway)
    {
        oldPos->second = sumPos;
        return true;
    }
    return false;
}

/*
 * Mapper
 */
RPLL_DamageHitType RPLLHooks::mapHitMaskToRPLLHitType(const uint32_t hitMask)
{
    // We will break down the HitMask into an enum
    if (hitMask & ProcFlagsHit::PROC_HIT_CRITICAL)
        return RPLL_DamageHitType::RPLL_CRIT;
    if (hitMask & ProcFlagsHit::PROC_HIT_MISS || hitMask & ProcFlagsHit::PROC_HIT_FULL_RESIST)
        return RPLL_DamageHitType::RPLL_MISS;
    if (hitMask & ProcFlagsHit::PROC_HIT_DODGE)
        return RPLL_DamageHitType::RPLL_DODGE;
    if (hitMask & ProcFlagsHit::PROC_HIT_PARRY)
        return RPLL_DamageHitType::RPLL_PARRY;
    if (hitMask & ProcFlagsHit::PROC_HIT_BLOCK || hitMask & ProcFlagsHit::PROC_HIT_FULL_BLOCK)
        return RPLL_DamageHitType::RPLL_BLOCK;
    if (hitMask & ProcFlagsHit::PROC_HIT_EVADE)
        return RPLL_DamageHitType::RPLL_EVADE;
    if (hitMask & ProcFlagsHit::PROC_HIT_IMMUNE)
        return RPLL_DamageHitType::RPLL_IMMUNE;
    if (hitMask & ProcFlagsHit::PROC_HIT_ABSORB)
        return RPLL_DamageHitType::RPLL_ABSORB;
    if (hitMask & ProcFlagsHit::PROC_HIT_INTERRUPT)
        return RPLL_DamageHitType::RPLL_INTERRUPT;
    // Default
    // PROC_HIT_NONE
    // PROC_HIT_NORMAL
    return RPLL_DamageHitType::RPLL_HIT;
}

RPLL_PowerType RPLLHooks::mapPowersToRPLLPowerType(const Powers power)
{
    switch (power)
    {
    case Powers::POWER_MANA:
        return RPLL_PowerType::RPLL_MANA;
    case Powers::POWER_RAGE:
        return RPLL_PowerType::RPLL_RAGE;
    case Powers::POWER_FOCUS:
        return RPLL_PowerType::RPLL_FOCUS;
    case Powers::POWER_ENERGY:
        return RPLL_PowerType::RPLL_ENERGY;
    case Powers::POWER_HAPPINESS:
        return RPLL_PowerType::RPLL_HAPPINESS;
    case Powers::POWER_HEALTH:
        return RPLL_PowerType::RPLL_HEALTH;
    }
    return RPLL_PowerType::RPLL_PWT_UNDEFINED;
}

RPLL_DamageSchool RPLLHooks::mapSpellSchoolToRPLLDamageSchool(const SpellSchools school)
{
    switch (school)
    {
    case SpellSchools::SPELL_SCHOOL_NORMAL:
        return RPLL_DamageSchool::RPLL_PHYSICAL;
    case SpellSchools::SPELL_SCHOOL_HOLY:
        return RPLL_DamageSchool::RPLL_HOLY;
    case SpellSchools::SPELL_SCHOOL_FIRE:
        return RPLL_DamageSchool::RPLL_FIRE;
    case SpellSchools::SPELL_SCHOOL_NATURE:
        return RPLL_DamageSchool::RPLL_NATURE;
    case SpellSchools::SPELL_SCHOOL_FROST:
        return RPLL_DamageSchool::RPLL_FROST;
    case SpellSchools::SPELL_SCHOOL_SHADOW:
        return RPLL_DamageSchool::RPLL_SHADOW;
    case SpellSchools::SPELL_SCHOOL_ARCANE:
        return RPLL_DamageSchool::RPLL_ARCANE;
    }
    return RPLL_DamageSchool::RPLL_DS_UNDEFINED;
}

RPLL_DamageSchool RPLLHooks::mapSpellSchoolMaskToRPLLDamageSchool(const uint32_t schoolMask)
{
    // Damage done in TC is split into each school for calcultion
    // Hence we will likely not get mixed results here
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_NORMAL)
        return RPLL_DamageSchool::RPLL_PHYSICAL;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_HOLY)
        return RPLL_DamageSchool::RPLL_HOLY;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_FIRE)
        return RPLL_DamageSchool::RPLL_FIRE;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_NATURE)
        return RPLL_DamageSchool::RPLL_NATURE;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_FROST)
        return RPLL_DamageSchool::RPLL_FROST;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_SHADOW)
        return RPLL_DamageSchool::RPLL_SHADOW;
    if (schoolMask & SpellSchoolMask::SPELL_SCHOOL_MASK_ARCANE)
        return RPLL_DamageSchool::RPLL_ARCANE;
    return RPLL_DamageSchool::RPLL_DS_UNDEFINED;
}

RPLL_PvP_Winner RPLLHooks::mapPvPWinnerToRPLLPvPWinner(const uint8_t winner)
{
    if (winner == TEAM_ALLIANCE)
    {
        return RPLL_PvP_Winner::RPLL_TEAM_ALLIANCE;
    }
    else if (winner == TEAM_HORDE)
    {
        return RPLL_PvP_Winner::RPLL_TEAM_HORDE;
    }
    return RPLL_PvP_Winner::RPLL_TEAM_NONE;
}

/*
 * Pack information and send messages
 */
void RPLLHooks::DealSpellDamage(const Unit *attacker, const Unit *victim, const uint32_t spellId, const uint32_t blocked, const RPLL_Damage damage)
{
    if (!IsInInstance(victim))
        return;
    const uint8_t msgLength = 24 + GetMessageMetaDataSize() + 13;
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_DAMAGE, msgLength);
    msg << static_cast<uint64_t>(attacker->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(victim->GetGUID().GetRawValue());
    msg << spellId;
    msg << blocked;
    AppendRPLLDamage(msg, damage);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::DealMeleeDamage(const Unit *attacker, const Unit *victim, const RPLL_DamageHitType damageHitType, const uint32_t blocked, const std::vector<RPLL_Damage> damages)
{
    if (!IsInInstance(victim))
        return;
    const uint8_t msgLength = 21 + GetMessageMetaDataSize() + damages.size() * 13;
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << static_cast<uint64_t>(attacker->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(victim->GetGUID().GetRawValue());
    msg << blocked;
    msg << static_cast<uint8_t>(damageHitType);
    for (auto dmg : damages)
        AppendRPLLDamage(msg, dmg);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::DealMeleeDamage(const Unit *attacker, const Unit *victim, const RPLL_DamageHitType damageHitType, const uint32_t blocked, const RPLL_Damage damage)
{
    if (!IsInInstance(victim))
        return;
    const uint8_t msgLength = 21 + GetMessageMetaDataSize() + sizeof(RPLL_Damage);
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << static_cast<uint64_t>(attacker->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(victim->GetGUID().GetRawValue());
    msg << blocked;
    msg << static_cast<uint8_t>(damageHitType);
    AppendRPLLDamage(msg, damage);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Heal(const Unit *caster, const Unit *target, const uint32_t spellId, const uint32_t totalHeal, const uint32_t effectiveHeal, const uint32_t absorb)
{
    if (!IsInInstance(target))
        return;
    const uint8_t msgLength = 32 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_HEAL, msgLength);
    msg << static_cast<uint64_t>(caster->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(target->GetGUID().GetRawValue());
    msg << spellId;
    msg << totalHeal;
    msg << effectiveHeal;
    msg << absorb; // ?
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Death(const Unit *cause, const Unit *victim)
{
    if (!IsInInstance(victim))
        return;
    const uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_DEATH, msgLength);
    msg << static_cast<uint64_t>(cause->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(victim->GetGUID().GetRawValue());
    SendZmqMessage(std::move(msg));
}

// Apllied = True => A stack has been added (Also refresh)
// Applied = False => A stack has been removed
void RPLLHooks::AuraApplication(const Unit *caster, const Unit *target, const uint32_t spellId, const uint32_t stackAmount, const bool applied)
{
    if (!IsInInstance(target))
        return;
    const uint8_t msgLength = 25 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_AURA_APPLICATION, msgLength);
    msg << static_cast<uint64_t>(caster->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(target->GetGUID().GetRawValue());
    msg << spellId;
    msg << stackAmount;
    msg << static_cast<uint8_t>(applied);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Dispel(const Unit *dispeller, const Unit *target, const ObjectGuid auraCaster, const uint32_t dispelSpellId, const uint32_t dispelledSpellId, const uint8_t dispelAmount)
{
    if (!IsInInstance(target))
        return;
    const uint8_t msgLength = 33 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_DISPEL, msgLength);
    msg << static_cast<uint64_t>(dispeller->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(target->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(auraCaster.GetRawValue());
    msg << dispelSpellId;
    msg << dispelledSpellId;
    msg << dispelAmount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::SpellSteal(const Unit *dispeller, const Unit *target, const ObjectGuid auraCaster, const uint32_t stealSpellId, const uint32_t stolenSpellId, const uint8_t stealAmount)
{
    if (!IsInInstance(target))
        return;
    const uint8_t msgLength = 33 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_STEAL, msgLength);
    msg << static_cast<uint64_t>(dispeller->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(target->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(auraCaster.GetRawValue());
    msg << stealSpellId;
    msg << stolenSpellId;
    msg << stealAmount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Interrupt(const Unit *target, const uint32_t interruptedSpellId)
{
    if (!IsInInstance(target))
        return;
    const uint8_t msgLength = 12 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INTERRUPT, msgLength);
    msg << static_cast<uint64_t>(target->GetGUID().GetRawValue());
    msg << interruptedSpellId;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Position(const Unit *unit, const float x, const float y, const float z, const float orientation)
{
    if (!HasSignificantPositionChange(unit, x, y, z, orientation))
        return;
    if (!IsPositionWithinTimeout(unit))
        return; // Implies is in Instance
    const uint8_t msgLength = 24 + GetMessageMetaDataSize() + GetMapMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_POSITION, msgLength);
    AppendMapMetaData(msg, unit);
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    msg << static_cast<int32_t>(x * 10);
    msg << static_cast<int32_t>(y * 10);
    msg << static_cast<int32_t>(z * 10);
    msg << static_cast<int32_t>(orientation * 10);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::CombatState(const Unit *unit, const bool inCombat)
{
    if (!IsInInstance(unit))
        return;
    const uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_COMBAT_STATE, msgLength);
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    msg << static_cast<uint8_t>(inCombat);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Power(const Unit *unit, const RPLL_PowerType powerType, const uint32_t maxPower, const uint32_t currentPower)
{
    if (!IsPowerWithinTimeout(unit, powerType))
        return; // This implies IsInInstance
    const uint8_t msgLength = 17 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_POWER, msgLength);
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    msg << static_cast<uint8_t>(powerType);
    msg << maxPower;
    msg << currentPower;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::StartBattleground(const uint32_t mapId, const uint32_t instanceId)
{
    RPLLHooks::Instance(mapId, instanceId, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_START_BATTLEGROUND);
}

void RPLLHooks::StartUnratedArena(const uint32_t mapId, const uint32_t instanceId)
{
    RPLLHooks::Instance(mapId, instanceId, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_START_UNRATED_ARENA);
}

void RPLLHooks::StartRatedArena(const uint32_t mapId, const uint32_t instanceId, const uint32_t teamId1, const uint32_t teamId2)
{
    const uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_START_RATED_ARENA, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << teamId1;
    msg << teamId2;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Instance(const uint32_t mapId, const uint32_t instanceId, const RPLL_MessageType messageType)
{
    const uint8_t msgLength = 8 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, messageType, msgLength);
    msg << mapId;
    msg << instanceId;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::EndUnratedArena(const uint32_t mapId, const uint32_t instanceId, const RPLL_PvP_Winner winner)
{
    const uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_UNRATED_ARENA, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << static_cast<uint8_t>(winner);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::EndBattleground(const uint32_t mapId, const uint32_t instanceId, const RPLL_PvP_Winner winner, const uint32_t scoreAlliance, const uint32_t scoreHorde)
{
    const uint8_t msgLength = 17 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_BATTLEGROUND, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << static_cast<uint8_t>(winner);
    msg << scoreAlliance;
    msg << scoreHorde;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::EndRatedArena(const uint32_t mapId, const uint32_t instanceId, const RPLL_PvP_Winner winner, const uint32_t teamId1, const uint32_t teamId2, const int32_t teamChange1, const int32_t teamChange2)
{
    const uint8_t msgLength = 25 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_RATED_ARENA, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << static_cast<uint8_t>(winner);
    msg << teamId1;
    msg << teamId2;
    msg << teamChange1;
    msg << teamChange2;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Loot(const Unit *unit, const uint32_t itemId, const uint32_t count)
{
    if (!IsInInstance(unit))
        return;
    const uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_LOOT, msgLength);
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    msg << itemId;
    msg << count;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::SpellCast(const Unit *caster, const uint64_t targetGUID, const uint32_t spellId, const RPLL_DamageHitType hitType)
{
    if (!IsInInstance(caster))
        return;
    const uint8_t msgLength = 21 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_CAST, msgLength);
    msg << static_cast<uint64_t>(caster->GetGUID().GetRawValue());
    msg << targetGUID; // Can be 0 for objects, like Rezz on a corpse
    msg << spellId;
    msg << static_cast<uint8_t>(hitType);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Threat(const Unit *threater, const Unit *threatened, const uint32_t spellId, const int32_t amount)
{
    if (!IsInInstance(threater))
        return;
    const uint8_t msgLength = 24 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_THREAT, msgLength);
    msg << static_cast<uint64_t>(threater->GetGUID().GetRawValue());
    msg << static_cast<uint64_t>(threatened->GetGUID().GetRawValue());
    msg << spellId;
    msg << amount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Event(const Unit *unit, const RPLL_Event event)
{
    if (!IsInInstance(unit))
        return;
    const uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_EVENT, msgLength);
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    msg << static_cast<uint8_t>(event);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Summon(const Unit *unit, const uint64_t ownerGUID)
{
    if (!IsInInstance(unit))
        return;
    const uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SUMMON, msgLength);
    msg << ownerGUID;
    msg << static_cast<uint64_t>(unit->GetGUID().GetRawValue());
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::InstanceDelete(const uint32_t instanceId)
{
    const uint8_t msgLength = 4 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_DELETE, msgLength);
    msg << instanceId;
    SendZmqMessage(std::move(msg));
}