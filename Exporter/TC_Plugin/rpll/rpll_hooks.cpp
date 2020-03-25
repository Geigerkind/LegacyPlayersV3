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
std::map<uint64_t, double> RPLLHooks::LAST_UNIT_POSITION = {};

std::map<uint64_t, uint64_t> RPLLHooks::LAST_HEALTH_UPDATE = {};
std::map<uint64_t, uint64_t> RPLLHooks::LAST_POWER_UPDATE = {};
std::map<uint64_t, uint64_t> RPLLHooks::LAST_POSITION_UPDATE = {};

bool RPLLHooks::DEBUG = false;
void RPLLHooks::PrintDebugMessage(const char* msg, bool overrule) {
    if (RPLLHooks::DEBUG || overrule)
        std::cout << "DEBUG: " << msg << std::endl;
}


/*
 * ZMQ Messaging
 */
void* RPLLHooks::zmqContext = nullptr;
void* RPLLHooks::zmqSocket = nullptr;
void* RPLLHooks::GetZmqSocket() {
    if (RPLLHooks::zmqSocket == nullptr) {
        std::cout << "Creating ZMQ Socket" << std::endl;
        RPLLHooks::zmqContext = zmq_ctx_new();
        RPLLHooks::zmqSocket = zmq_socket(RPLLHooks::zmqContext, ZMQ_PUSH);
        // Address of the Backend docker container
        int rc = zmq_connect(RPLLHooks::zmqSocket, "tcp://172.20.128.5:5690");
        assert (rc == 0);
    }
    return RPLLHooks::zmqSocket;
}

void RPLLHooks::SendZmqMessage(ByteBuffer &&msg) {
    zmq_send(GetZmqSocket(), msg.contents(), msg.size(), ZMQ_DONTWAIT);
    PrintMessage(std::move(msg));
}

void RPLLHooks::PrintMessage(ByteBuffer &&msg) {
    std::cout << "RPLL: [ ";
    for (size_t i = 0; i < msg.size(); ++i)
        std::cout << uint32_t(msg[i]) << " ";
    std::cout << "] (" << msg.size() << ")" << std::endl;
}

/*
 * Time
 */
uint8_t RPLLHooks::GetCurrentTimeSize() {
    return sizeof(uint64_t);
}
uint64_t RPLLHooks::GetCurrentTime() {
    return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count());
}

/*
 * Conditions
 */
std::set<uint32_t> RPLLHooks::raidMapIds = {
    249, 309, 409, 469, 509, 531, // Vanilla
    532, 533, 534, 544, 548, 550, 564, 565, 568, 580, // TBC
    603, 615, 616, 624, 631, 649, 724 // WOTLK
};
bool RPLLHooks::IsInRaid(Unit* unit) {
    return RPLLHooks::raidMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::raidMapIds.end();
}

std::set<uint32_t> RPLLHooks::arenaMapIds = {
    559, 562, 572, 617, 618
};
bool RPLLHooks::IsInArena(Unit* unit) {
    return RPLLHooks::arenaMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::arenaMapIds.end();
}

std::set<uint32_t> RPLLHooks::battlegroundMapIds = {
    30, 489, 529, 566, 607, 628
};
bool RPLLHooks::IsInBattleground(Unit* unit) {
    return RPLLHooks::battlegroundMapIds.find(unit->GetMap()->GetId()) != RPLLHooks::battlegroundMapIds.end();
}

bool RPLLHooks::IsInInstance(Unit* unit) {
    return IsInRaid(unit) || IsInArena(unit) || IsInBattleground(unit);
}


/*
 * Health/Power data
 */
uint8_t RPLLHooks::GetMapMetaDataSize() {
    return 2 * sizeof(uint32_t) + sizeof(uint8_t);
}
void RPLLHooks::AppendMapMetaData(ByteBuffer &msg, Unit* unit) {
    Map* map = unit->GetMap();
    msg << uint32_t(map->GetId());
    msg << uint32_t(map->GetInstanceId());
    msg << uint8_t(map->GetDifficulty());
}

/*
 * Message Meta Data
 */
uint8_t RPLLHooks::GetMessageMetaDataSize() {
    return 3 * sizeof(uint8_t) + GetCurrentTimeSize();
}
void RPLLHooks::AppendMessageMetaData(ByteBuffer &msg, uint8_t msgType, uint8_t msgLength) {
    msg << API_VERSION;
    msg << msgType;
    msg << msgLength;
    msg << GetCurrentTime();
}

/*
 * Helper
 */

RPLL_Damage RPLLHooks::BuildRPLLDamage(RPLL_DamageSchool damageSchool, uint32_t damage, uint32_t resisted_or_glanced, uint32_t absorbed) {
    RPLL_Damage dmg;
    dmg.damageSchool = damageSchool;
    dmg.damage = damage;
    dmg.resisted_or_glanced = resisted_or_glanced;
    dmg.absorbed = absorbed;
    return std::move(dmg);
}

void RPLLHooks::AppendRPLLDamage(ByteBuffer &msg, RPLL_Damage &damage) {
    msg << uint8_t(damage.damageSchool);
    msg << damage.damage;
    msg << damage.resisted_or_glanced;
    msg << damage.absorbed;
}

/*
 * Methods used to optimize
 */
bool RPLLHooks::IsPowerWithinTimeout(Unit* unit, RPLL_PowerType power) {
    // Dont judge me
    // I was too lazy to extract a method here
    if (IsInRaid(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        if (power == RPLL_PowerType::RPLL_HEALTH) {
            auto res = RPLLHooks::LAST_HEALTH_UPDATE.find(guid);
            if (res == RPLLHooks::LAST_HEALTH_UPDATE.end() || (now - res->second) >= RPLLHooks::RAID_UPDATE_TIMEOUT) {
                RPLLHooks::LAST_HEALTH_UPDATE[guid] = now;
                return true;
            }
            return false;
        }
        auto res = RPLLHooks::LAST_POWER_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POWER_UPDATE.end() || (now - res->second) >= RPLLHooks::RAID_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POWER_UPDATE[guid] = now;
            return true;
        }
        return false;
    } else if (IsInArena(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        if (power == RPLL_PowerType::RPLL_HEALTH) {
            auto res = RPLLHooks::LAST_HEALTH_UPDATE.find(guid);
            if (res == RPLLHooks::LAST_HEALTH_UPDATE.end() || (now - res->second) >= RPLLHooks::ARENA_UPDATE_TIMEOUT) {
                RPLLHooks::LAST_HEALTH_UPDATE[guid] = now;
                return true;
            }
            return false;
        }
        auto res = RPLLHooks::LAST_POWER_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POWER_UPDATE.end() || (now - res->second) >= RPLLHooks::ARENA_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POWER_UPDATE[guid] = now;
            return true;
        }
        return false;
    } else if (IsInBattleground(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        if (power == RPLL_PowerType::RPLL_HEALTH) {
            auto res = RPLLHooks::LAST_HEALTH_UPDATE.find(guid);
            if (res == RPLLHooks::LAST_HEALTH_UPDATE.end() || (now - res->second) >= RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT) {
                RPLLHooks::LAST_HEALTH_UPDATE[guid] = now;
                return true;
            }
            return false;
        }
        auto res = RPLLHooks::LAST_POWER_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POWER_UPDATE.end() || (now - res->second) >= RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POWER_UPDATE[guid] = now;
            return true;
        }
        return false;
    }
    return false;
}

bool RPLLHooks::IsPositionWithinTimeout(Unit* unit) {
    if (IsInRaid(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        auto res = RPLLHooks::LAST_POSITION_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POSITION_UPDATE.end() || (now - res->second) >= RPLLHooks::RAID_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POSITION_UPDATE[guid] = now;
            return true;
        }
        return false;
    } else if (IsInArena(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        auto res = RPLLHooks::LAST_POSITION_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POSITION_UPDATE.end() || (now - res->second) >= RPLLHooks::ARENA_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POSITION_UPDATE[guid] = now;
            return true;
        }
        return false;
    } else if (IsInBattleground(unit)) {
        auto now = GetCurrentTime();
        auto guid = unit->GetGUID().GetRawValue();
        auto res = RPLLHooks::LAST_POSITION_UPDATE.find(guid);
        if (res == RPLLHooks::LAST_POSITION_UPDATE.end() || (now - res->second) >= RPLLHooks::BATTLEGROUND_UPDATE_TIMEOUT) {
            RPLLHooks::LAST_POSITION_UPDATE[guid] = now;
            return true;
        }
        return false;
    }
    return false;
}

bool RPLLHooks::HasSignificantPositionChange(Unit* unit, float x, float y, float z, float orientation) {
    uint64_t unitGuid = unit->GetGUID().GetRawValue();
    auto oldPos = RPLLHooks::LAST_UNIT_POSITION.find(unitGuid);
    double sumPos = std::abs(x) + std::abs(y) + std::abs(z) + std::abs(orientation);
    if (IsInArena(unit)) {
        if (oldPos == RPLLHooks::LAST_UNIT_POSITION.end() || std::fabs(oldPos->second - sumPos) >= RPLLHooks::UPDATE_POSITION_LEEWAY_ARENA) {
            RPLLHooks::LAST_UNIT_POSITION[unitGuid] = sumPos;
            return true;
        }
        return false;
    }
    if (oldPos == RPLLHooks::LAST_UNIT_POSITION.end() || std::fabs(oldPos->second - sumPos) >= RPLLHooks::UPDATE_POSITION_LEEWAY) {
        RPLLHooks::LAST_UNIT_POSITION[unitGuid] = sumPos;
        return true;
    }
    return false;
}

/*
 * Mapper
 */
RPLL_DamageHitType RPLLHooks::mapHitMaskToRPLLHitType(uint32_t hitMask) {
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

RPLL_PowerType RPLLHooks::mapPowersToRPLLPowerType(Powers power) {
    switch (power) {
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

RPLL_DamageSchool RPLLHooks::mapSpellSchoolToRPLLDamageSchool(SpellSchools school) {
    switch (school) {
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

RPLL_DamageSchool RPLLHooks::mapSpellSchoolMaskToRPLLDamageSchool(uint32_t schoolMask) {
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

RPLL_PvP_Winner RPLLHooks::mapPvPWinnerToRPLLPvPWinner(uint8_t winner) {
    if (winner == TEAM_ALLIANCE) {
        return RPLL_PvP_Winner::RPLL_TEAM_ALLIANCE;
    } else if (winner == TEAM_HORDE) {
        return RPLL_PvP_Winner::RPLL_TEAM_HORDE;
    }
    return RPLL_PvP_Winner::RPLL_TEAM_NONE;
}

/*
 * Pack information and send messages
 */
void RPLLHooks::DealSpellDamage(Unit* attacker, Unit* victim, uint32_t spellId, uint32_t blocked, RPLL_Damage&& damage) {
    if (!IsInInstance(victim)) return;
    uint8_t msgLength = 24 + GetMessageMetaDataSize() + sizeof(RPLL_Damage);
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_DAMAGE, msgLength);
    msg << uint64_t(attacker->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    msg << spellId;
    msg << blocked;
    AppendRPLLDamage(msg, damage);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::DealMeleeDamage(Unit* attacker, Unit* victim, RPLL_DamageHitType damageHitType, uint32_t blocked, std::vector<RPLL_Damage>&& damages) {
    if (!IsInInstance(victim)) return;
    uint8_t msgLength = 20 + GetMessageMetaDataSize() + damages.size() * sizeof(RPLL_Damage);
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << uint64_t(attacker->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    msg << blocked;
    for (auto dmg : damages)
        AppendRPLLDamage(msg, dmg);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::DealMeleeDamage(Unit* attacker, Unit* victim, RPLL_DamageHitType damageHitType, uint32_t blocked, RPLL_Damage&& damage) {
    if (!IsInInstance(victim)) return;
    uint8_t msgLength = 20 + GetMessageMetaDataSize() + sizeof(RPLL_Damage);
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << uint64_t(attacker->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    msg << blocked;
    AppendRPLLDamage(msg, damage);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Heal(Unit* caster, Unit* target, uint32_t spellId, uint32_t totalHeal, uint32_t effectiveHeal, uint32_t absorb) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 33 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_HEAL, msgLength);
    msg << uint64_t(caster->GetGUID().GetRawValue());
    msg << uint64_t(target->GetGUID().GetRawValue());
    msg << spellId;
    msg << totalHeal;
    msg << effectiveHeal;
    msg << absorb; // ?
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Death(Unit* cause, Unit* victim) {
    if (!IsInInstance(victim)) return;
    uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_DEATH, msgLength);
    msg << uint64_t(cause->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    SendZmqMessage(std::move(msg));
}

// Apllied = True => A stack has been added (Also refresh)
// Applied = False => A stack has been removed
void RPLLHooks::AuraApplication(Unit* caster, Unit* target, uint32_t spellId, uint32_t stackAmount, bool applied) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 25 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_AURA_APPLICATION, msgLength);
    msg << uint64_t(caster->GetGUID().GetRawValue());
    msg << uint64_t(target->GetGUID().GetRawValue());
    msg << spellId;
    msg << stackAmount;
    msg << uint8_t(applied);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Dispel(Unit* dispeller, Unit* target, ObjectGuid auraCaster, uint32_t dispelSpellId, uint32_t dispelledSpellId, uint8_t dispelAmount) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 33 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_DISPEL, msgLength);
    msg << uint64_t(dispeller->GetGUID().GetRawValue());
    msg << uint64_t(target->GetGUID().GetRawValue());
    msg << uint64_t(auraCaster.GetRawValue());
    msg << dispelSpellId;
    msg << dispelledSpellId;
    msg << dispelAmount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::SpellSteal(Unit* dispeller, Unit* target, ObjectGuid auraCaster, uint32_t stealSpellId, uint32_t stolenSpellId, uint8_t stealAmount) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 33 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_STEAL, msgLength);
    msg << uint64_t(dispeller->GetGUID().GetRawValue());
    msg << uint64_t(target->GetGUID().GetRawValue());
    msg << uint64_t(auraCaster.GetRawValue());
    msg << stealSpellId;
    msg << stolenSpellId;
    msg << stealAmount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Interrupt(Unit* target, uint32_t interruptedSpellId) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 12 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INTERRUPT, msgLength);
    msg << uint64_t(target->GetGUID().GetRawValue());
    msg << interruptedSpellId;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Position(Unit* unit, float x, float y, float z, float orientation) {
    if (!HasSignificantPositionChange(unit, x, y, z, orientation)) return;
    if (!IsPositionWithinTimeout(unit)) return; // Implies is in Instance
    uint8_t msgLength = 24 + GetMessageMetaDataSize() + GetMapMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_POSITION, msgLength);
    AppendMapMetaData(msg, unit);
    msg << uint64_t(unit->GetGUID().GetRawValue());
    msg << static_cast<int32_t>(x*10);
    msg << static_cast<int32_t>(y*10);
    msg << static_cast<int32_t>(z*10);
    msg << static_cast<int32_t>(orientation*10);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::CombatState(Unit* unit, bool inCombat) {
    if (!IsInInstance(unit)) return;
    uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_COMBAT_STATE, msgLength);
    msg << uint64_t(unit->GetGUID().GetRawValue());
    msg << uint8_t(inCombat);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Power(Unit * unit, RPLL_PowerType powerType, uint32_t maxPower, uint32_t currentPower) {
    if (!IsPowerWithinTimeout(unit, powerType)) return; // This implies IsInInstance
    uint8_t msgLength = 17 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_POWER, msgLength);
    msg << uint64_t(unit->GetGUID().GetRawValue());
    msg << uint8_t(powerType);
    msg << maxPower;
    msg << currentPower;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Instance(uint32_t mapId, uint32_t instanceId) {
    uint8_t msgLength = 8 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_START, msgLength);
    msg << mapId;
    msg << instanceId;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner) {
    uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_UNRATED_ARENA, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << uint8_t(winner);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t scoreAlliance, uint32_t scoreHorde) {
    uint8_t msgLength = 17 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_BATTLEGROUND, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << uint8_t(winner);
    msg << scoreAlliance;
    msg << scoreHorde;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t teamId1, uint32_t teamId2, int32_t teamChange1, int32_t teamChange2) {
    uint8_t msgLength = 25 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_INSTANCE_PVP_END_RATED_ARENA, msgLength);
    msg << mapId;
    msg << instanceId;
    msg << uint8_t(winner);
    msg << teamId1;
    msg << teamId2;
    msg << teamChange1;
    msg << teamChange2;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Loot(Unit* unit, uint32_t itemId) {
    if (!IsInInstance(unit)) return;
    uint8_t msgLength = 12 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_LOOT, msgLength);
    msg << uint64_t(unit->GetGUID().GetRawValue());
    msg << itemId;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::SpellCast(Unit* caster, uint64_t targetGUID, uint32_t spellId, RPLL_DamageHitType hitType) {
    if (!IsInInstance(caster)) return;
    uint8_t msgLength = 20 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SPELL_CAST, msgLength);
    msg << uint64_t(caster->GetGUID().GetRawValue());
    msg << targetGUID; // Can be 0 for objects, like Rezz on a corpse
    msg << spellId;
    msg << uint8_t(hitType);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Threat(Unit* threater, Unit* threatened, uint32_t spellId, int32_t amount) {
    if (!IsInInstance(threater)) return;
    uint8_t msgLength = 24 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_THREAT, msgLength);
    msg << uint64_t(threater->GetGUID().GetRawValue());
    msg << uint64_t(threatened->GetGUID().GetRawValue());
    msg << spellId;
    msg << amount;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Event(Unit* unit, RPLL_Event event) {
    if (!IsInInstance(unit)) return;
    uint8_t msgLength = 9 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_EVENT, msgLength);
    msg << uint64_t(unit->GetGUID().GetRawValue());
    msg << event;
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Summon(Unit* unit, uint64_t ownerGUID) {
    if (!IsInInstance(unit)) return;    
    uint8_t msgLength = 16 + GetMessageMetaDataSize();
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_SUMMON, msgLength);
    msg << ownerGUID;
    msg << uint64_t(unit->GetGUID().GetRawValue());
    SendZmqMessage(std::move(msg));
}


/*
 * Hook methods from Unit
 */
void RPLLHooks::SendAttackStateUpdate(CalcDamageInfo *damageInfo) {
    //PrintDebugMessage("SendAttackStateUpdate");
    if (damageInfo == nullptr || damageInfo->Attacker == nullptr || damageInfo->Target == nullptr)
        return;
    RPLL_DamageHitType damageHitType = mapHitMaskToRPLLHitType(damageInfo->HitInfo);
    std::vector<RPLL_Damage> damages;
    damages.reserve(1);
    for (auto dmg : damageInfo->Damages) {
        if (dmg.Damage == 0 && dmg.Absorb == 0 && dmg.Resist == 0)
            continue;
        RPLL_DamageSchool damageSchool = mapSpellSchoolMaskToRPLLDamageSchool(dmg.DamageSchoolMask);
        damages.push_back(std::move(BuildRPLLDamage(damageSchool, uint32_t(dmg.Damage), uint32_t(dmg.Resist), uint32_t(dmg.Absorb))));
    }
    DealMeleeDamage(damageInfo->Attacker, damageInfo->Target, damageHitType, uint32_t(damageInfo->Blocked), std::move(damages));
}

void RPLLHooks::SendSpellNonMeleeDamageLog(SpellNonMeleeDamage *damageInfo) {
    //PrintDebugMessage("SendSpellNonMeleeDamageLog");
    if (damageInfo == nullptr || damageInfo->attacker == nullptr || damageInfo->target == nullptr)
        return;

    RPLL_DamageSchool damageSchool = mapSpellSchoolMaskToRPLLDamageSchool(damageInfo->schoolMask);
    DealSpellDamage(damageInfo->attacker, damageInfo->target,
        uint32_t(damageInfo->blocked), uint32_t(damageInfo->SpellID),
        std::move(BuildRPLLDamage(damageSchool, uint32_t(damageInfo->damage), uint32_t(damageInfo->absorb), uint32_t(damageInfo->resist))));
}

void RPLLHooks::DealHeal(HealInfo& healInfo) {
    //PrintDebugMessage("DealHeal");
    Heal(healInfo.GetHealer(), healInfo.GetTarget(), healInfo.GetSpellInfo()->Id, healInfo.GetHeal(), healInfo.GetEffectiveHeal(), healInfo.GetAbsorb());
}

void RPLLHooks::SendPeriodicAuraLog(SpellPeriodicAuraLogInfo* pInfo) {
    //PrintDebugMessage("SendPeriodicAuraLog");
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
                RPLL_DamageSchool damageSchool = mapSpellSchoolMaskToRPLLDamageSchool(uint32_t(aura->GetSpellInfo()->SchoolMask)); 
                DealSpellDamage(caster, target, 0, aura->GetId(),
                    std::move(BuildRPLLDamage(damageSchool, uint32_t(pInfo->damage), uint32_t(pInfo->absorb), uint32_t(pInfo->resist))));
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

void RPLLHooks::Kill(Unit* attacker, Unit* victim) {
    //PrintDebugMessage("Kill");
    if (attacker != nullptr && victim != nullptr)
        Death(attacker, victim);
}

void RPLLHooks::RemoveAurasDueToSpellByDispel(Unit* target, uint32_t spellId, uint32_t dispellerSpellId, ObjectGuid casterGUID, WorldObject* dispeller, uint8 chargesRemoved) {
    //PrintDebugMessage("RemoveAurasDueToSpellByDispel");
    if (casterGUID.IsUnit() && dispeller->GetGUID().IsUnit() && dispeller->ToUnit() != nullptr)
        Dispel(dispeller->ToUnit(), target, casterGUID, dispellerSpellId, spellId, chargesRemoved);
}

void RPLLHooks::RemoveAurasDueToSpellBySteal(Unit* target, uint32_t spellId, uint32_t stealSpellId, ObjectGuid casterGUID, WorldObject* stealer) {
    //PrintDebugMessage("RemoveAurasDueToSpellBySteal");
    if (casterGUID.IsUnit() && stealer->GetGUID().IsUnit() && stealer->ToUnit() != nullptr)
        SpellSteal(stealer->ToUnit(), target, casterGUID, stealSpellId, spellId, 1);
}

void RPLLHooks::UpdatePosition(Unit* unit, float x, float y, float z, float orientation) {
    //PrintDebugMessage("UpdatePosition");
    if (unit == nullptr)
        return;
    Position(unit,x,y,z,orientation);
}

// Upon getting into PoV of the player the health is set.
// So while we may loose some precision, we get rid of a lot redundant events
void RPLLHooks::SetHealth(Unit* unit, uint32_t oldVal) {
    //PrintDebugMessage("SetHealth");
    if (unit == nullptr || oldVal == 0 || unit->GetHealth() == oldVal)
        return;
    Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
}

void RPLLHooks::SetMaxHealth(Unit* unit, uint32_t oldVal) {
    //PrintDebugMessage("SetMaxHealth");
    if (unit == nullptr || oldVal == 0 || unit->GetMaxHealth() == oldVal)
        return;
    Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
}

void RPLLHooks::SetPower(Unit* unit, Powers powerType, uint32_t oldVal) {
    //PrintDebugMessage("SetPower");
    if (unit == nullptr || oldVal == 0 || unit->GetPower(powerType) == oldVal)
        return;
    Power(unit, mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}

void RPLLHooks::SetMaxPower(Unit* unit, Powers powerType, uint32_t oldVal) {
    //PrintDebugMessage("SetMaxPower");
    if (unit == nullptr || oldVal == 0 || unit->GetMaxPower(powerType) == oldVal)
        return;
    Power(unit, mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}

void RPLLHooks::RemoveOwnedAura(Aura* aura) {
    //PrintDebugMessage("RemoveOwnedAura");
    if (aura == nullptr)
        return;
    auto owner = aura->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    AuraApplication(aura->GetCaster(), owner->ToUnit(), uint32_t(aura->GetId()), 0, false);
}

void RPLLHooks::SetOwnerGUID(Unit* unit, ObjectGuid owner) {
    //PrintDebugMessage("SetOwnerGUID");
    if (unit == nullptr)
        return;
    if (!unit->GetGUID().IsUnit() || !owner.IsUnit())
        return;
    Summon(unit, owner.GetRawValue());
}

/*
 * Hook Methods from CombatManager
 */
void RPLLHooks::UpdateOwnerCombatState(Unit* unit, bool result) {
    //PrintDebugMessage("UpdateOwnerCombatState");
    if (!result || unit == nullptr)
        return;
    CombatState(unit, unit->IsInCombat());
    Power(unit, RPLL_PowerType::RPLL_HEALTH, uint32_t(unit->GetMaxHealth()), uint32_t(unit->GetHealth()));
    auto powerType = unit->GetPowerType();
    Power(unit, mapPowersToRPLLPowerType(powerType), uint32_t(unit->GetMaxPower(powerType)), uint32_t(unit->GetPower(powerType)));
}

/*
 * Hook Methods from Map
 */
void RPLLHooks::StartBattleground(Battleground* battleground) {
    //PrintDebugMessage("StartBattleground");
    if (battleground == nullptr)
        return;
    Instance(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()));
}

void RPLLHooks::EndBattleground(Battleground* battleground, uint32_t *scores) {
    //PrintDebugMessage("EndBattleground");
    if (battleground == nullptr || scores == nullptr)
        return;
    RPLL_PvP_Winner winner = mapPvPWinnerToRPLLPvPWinner(battleground->GetWinner());
    if (battleground->isBattleground()) {
        uint32_t scoreAlliance = scores[TEAM_ALLIANCE];
        uint32_t scoreHorde = scores[TEAM_HORDE];
        Instance(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner, scoreAlliance, scoreHorde);
    } else if (battleground->isRated()) {
        uint32_t teamId1 = battleground->GetArenaTeamIdForTeam(ALLIANCE);
        uint32_t teamId2 = battleground->GetArenaTeamIdForTeam(HORDE);
        int32_t teamChange1 = battleground->GetArenaTeamRatingChangeForTeam(ALLIANCE);
        int32_t teamChange2 = battleground->GetArenaTeamRatingChangeForTeam(HORDE);
        Instance(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner, teamId1, teamId2, teamChange1, teamChange2);
    } else {
        Instance(uint32_t(battleground->GetMapId()), uint32_t(battleground->GetInstanceID()), winner);
    }
}

/*
 * Hook methods from Player
 */
void RPLLHooks::SendNewItem(Unit* unit, Item *item, uint32_t count, bool received, bool created, bool broadcast, bool sendChatMessage) {
    //PrintDebugMessage("SendNewItem");
    if (item == nullptr || unit == nullptr)
        return;
    
    if (!created && (broadcast || sendChatMessage) && count == 1 && (received || !received)) // That is the question !! 
        Loot(unit, item->GetEntry());
}

void RPLLHooks::EnvironmentalDamage(Unit* unit, EnviromentalDamage type, uint32_t damage, uint32_t result) {
    //PrintDebugMessage("EnvironmentalDamage");
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
    DealMeleeDamage(unit, unit, RPLL_DamageHitType::RPLL_ENVIRONMENT, 0,
        std::move(BuildRPLLDamage(damageSchool, result, resist, absorb)));
}

void RPLLHooks::SetMap(Unit* unit) {
    //PrintDebugMessage("SetMap");
    if (!IsInInstance(unit))
        return;
    Position(unit,unit->GetPositionX(), unit->GetPositionY(), unit->GetPositionZ(), unit->GetOrientation());
    for (auto aura : unit->GetOwnedAuras())
        AuraCreate(aura.second);
    Summon(unit, unit->GetOwnerGUID().GetRawValue()); // TODO: Does that crash?
}

/*
 * Hook methods from Spell
 */
void RPLLHooks::SendCastResult(const Spell* spell, SpellCastResult result) {
    //PrintDebugMessage("SendCastResult");
    if (spell == nullptr || result != SpellCastResult::SPELL_FAILED_INTERRUPTED)
        return;

    auto caster = spell->GetCaster();
    if (!caster->GetGUID().IsUnit())
        return;
    Interrupt(caster->ToUnit(), spell->GetSpellInfo()->Id);
}

void RPLLHooks::DoDamageAndTriggers(const Spell* spell, uint32 hitMask) {
    //PrintDebugMessage("DoDamageAndTriggers");
    auto caster = spell->GetCaster();
    if (!caster->GetGUID().IsUnit())
        return;
    
    RPLL_DamageHitType damageHitType = mapHitMaskToRPLLHitType(hitMask);

    auto unitTarget = spell->m_targets.GetUnitTarget();
    if (unitTarget != nullptr) {
        SpellCast(caster->ToUnit(), unitTarget->GetGUID().GetRawValue(), spell->GetSpellInfo()->Id, damageHitType);
        return;
    }
    if (spell->m_targets.GetCorpseTarget() != nullptr
        || spell->m_targets.GetItemTarget() != nullptr
        || spell->m_targets.GetObjectTarget() != nullptr
        || spell->m_targets.GetGOTarget() != nullptr
    ) {
        SpellCast(caster->ToUnit(), 0, spell->GetSpellInfo()->Id, damageHitType);
        return;
    }

    // Assume self cast
    SpellCast(caster->ToUnit(), caster->GetGUID().GetRawValue(), spell->GetSpellInfo()->Id, damageHitType);
}

/*
 * Hook methods from SpellAuras
 */
void RPLLHooks::AuraCreate(Aura* result) {
    //PrintDebugMessage("AuraCreate");
    if (result == nullptr)
        return;
    auto owner = result->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    AuraApplication(result->GetCaster(), owner->ToUnit(), uint32_t(result->GetId()), uint32_t(result->GetStackAmount()), true);
}

void RPLLHooks::AuraSetStackAmount(Aura* aura, uint32_t oldAmount) {
    //PrintDebugMessage("AuraSetStackAmount");
    if (aura == nullptr)
        return;
    auto owner = aura->GetOwner();
    if (owner == nullptr || !owner->GetGUID().IsUnit())
        return;
    
    auto amount = uint32_t(aura->GetStackAmount());
    if (amount != oldAmount)
        AuraApplication(aura->GetCaster(), owner->ToUnit(), uint32_t(aura->GetId()), uint32_t(aura->GetStackAmount()), amount > oldAmount);
}

/*
 * Hook methods from ThreatManager
 */
void RPLLHooks::AddThreat(Unit* owner, Unit* target, SpellInfo const* spell, float amountBefore, float amountAfter) {
    //PrintDebugMessage("AddThreat");
    if (owner == nullptr || target == nullptr || std::fabs(amountAfter-amountBefore) <= 0.01f)
        return;
    Threat(target, owner, spell == nullptr ? 0 : spell->Id, static_cast<int32_t>((amountAfter-amountBefore)*10));
}

void RPLLHooks::ScaleThreat(Unit* owner, Unit* target, float factor) {
    //PrintDebugMessage("ScaleThreat");
    if (owner == nullptr || target == nullptr)
        return;
    if (factor <= 0.01f && owner->GetGUID().IsCreature()
        && (owner->ToCreature()->IsWorldBoss() || owner->ToCreature()->IsDungeonBoss())
    ) {
        Event(target, RPLL_Event::RPLL_THREAT_WIPE);
    }
}