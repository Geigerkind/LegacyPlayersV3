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
        int rc = zmq_connect(RPLLHooks::zmqSocket, "tcp://172.32.128.5:5690");
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
    uint8_t msgLength = 24 + GetMessageMetaDataSize() + 13;
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
    uint8_t msgLength = 21 + GetMessageMetaDataSize() + damages.size() * 13;
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << uint64_t(attacker->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    msg << blocked;
    msg << uint8_t(damageHitType);
    for (auto dmg : damages)
        AppendRPLLDamage(msg, dmg);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::DealMeleeDamage(Unit* attacker, Unit* victim, RPLL_DamageHitType damageHitType, uint32_t blocked, RPLL_Damage&& damage) {
    if (!IsInInstance(victim)) return;
    uint8_t msgLength = 21 + GetMessageMetaDataSize() + sizeof(RPLL_Damage);
    ByteBuffer msg(msgLength);
    AppendMessageMetaData(msg, RPLL_MessageType::RPLL_MSG_MELEE_DAMAGE, msgLength);
    msg << uint64_t(attacker->GetGUID().GetRawValue());
    msg << uint64_t(victim->GetGUID().GetRawValue());
    msg << blocked;
    msg << uint8_t(damageHitType);
    AppendRPLLDamage(msg, damage);
    SendZmqMessage(std::move(msg));
}

void RPLLHooks::Heal(Unit* caster, Unit* target, uint32_t spellId, uint32_t totalHeal, uint32_t effectiveHeal, uint32_t absorb) {
    if (!IsInInstance(target)) return;
    uint8_t msgLength = 32 + GetMessageMetaDataSize();
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
    uint8_t msgLength = 21 + GetMessageMetaDataSize();
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