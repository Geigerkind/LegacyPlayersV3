#ifndef _RPLL_HOOKS_H
#define _RPLL_HOOKS_H

#include <zmq.h>
#include <cinttypes>
#include "Unit.h"
#include "ByteBuffer.h"

enum class RPLL_MessageType : uint8_t
{
    RPLL_MSG_MELEE_DAMAGE = 0,                      // Works
    RPLL_MSG_SPELL_DAMAGE = 1,                      // Works
    RPLL_MSG_HEAL = 2,                              // Works
    RPLL_MSG_DEATH = 3,                             // Works
    RPLL_MSG_AURA_APPLICATION = 4,                  // Works
    RPLL_MSG_DISPEL = 5,                            // Works
    RPLL_MSG_SPELL_STEAL = 6,                       // Works, but needs post analysis
    RPLL_MSG_INTERRUPT = 7,                         // Works, but needs post analysis
    RPLL_MSG_POSITION = 8,                          // Works
    RPLL_MSG_COMBAT_STATE = 9,                      // Works
    RPLL_MSG_POWER = 10,                            // Works
    RPLL_MSG_LOOT = 11,                             // Works
    RPLL_MSG_SPELL_CAST = 12,                       // Works
    RPLL_MSG_THREAT = 13,                           // Works
    RPLL_MSG_EVENT = 14,                            // See events
    RPLL_MSG_SUMMON = 15,                           // Works
    RPLL_MSG_INSTANCE_PVP_START_UNRATED_ARENA = 16, // Works
    RPLL_MSG_INSTANCE_PVP_START_RATED_ARENA = 17,   // Works
    RPLL_MSG_INSTANCE_PVP_START_BATTLEGROUND = 18,  // Works
    RPLL_MSG_INSTANCE_PVP_END_UNRATED_ARENA = 19,   // Works
    RPLL_MSG_INSTANCE_PVP_END_RATED_ARENA = 20,     // Works
    RPLL_MSG_INSTANCE_PVP_END_BATTLEGROUND = 21,     // Works
    RPLL_MSG_INSTANCE_DELETE = 22
};

enum class RPLL_Event : uint8_t
{
    RPLL_THREAT_WIPE // Does not work!
};

enum class RPLL_DamageHitType : uint8_t
{
    RPLL_EVADE = 0,
    RPLL_MISS = 1,
    RPLL_DODGE = 2,
    RPLL_BLOCK = 3,
    RPLL_PARRY = 4,
    RPLL_GLANCING = 5,
    RPLL_CRIT = 6,
    RPLL_CRUSHING = 7,
    RPLL_HIT = 8,
    RPLL_RESIST = 9,
    RPLL_IMMUNE = 10,
    RPLL_ENVIRONMENT = 11,
    RPLL_ABSORB = 12,
    RPLL_INTERRUPT = 13,
    RPLL_DHT_UNDEFINED = 255
};

enum class RPLL_DamageSchool : uint8_t
{
    RPLL_PHYSICAL = 0,
    RPLL_HOLY = 1,
    RPLL_FIRE = 2,
    RPLL_NATURE = 3,
    RPLL_FROST = 4,
    RPLL_SHADOW = 5,
    RPLL_ARCANE = 6,
    RPLL_DS_UNDEFINED = 255
};

enum class RPLL_PowerType : uint8_t
{
    RPLL_MANA = 0,
    RPLL_RAGE = 1,
    RPLL_FOCUS = 2,
    RPLL_ENERGY = 3,
    RPLL_HAPPINESS = 4,
    RPLL_HEALTH = 5,
    RPLL_PWT_UNDEFINED = 255
};

enum class RPLL_PvP_Winner : uint8_t
{
    RPLL_TEAM_NONE = 0,
    RPLL_TEAM_HORDE = 1,
    RPLL_TEAM_ALLIANCE = 2
};

struct RPLL_Damage
{
    RPLL_DamageSchool damageSchool;
    uint32_t damage;
    uint32_t resisted_or_glanced;
    uint32_t absorbed;
};

// Help structure
struct RPLL_LastUpdate
{
    uint64_t health;
    uint64_t power;
    uint64_t position;
};

/*
 * HOW TO INSTALL
 * Below you will find some public methods.
 * These methods are the actual send methods used to transmit the data to the exporting docker container.
 * 
 * You have 2 choices. If your core is very similar to the Stock version
 * of the reference core I used (https://github.com/TrinityTBC/core)
 * then you implement all the hooks from the following files:
 * - rpll_battleground_hooks
 * - rpll_unit_hooks
 * - rpll_player_hooks
 * - rpll_spell_hooks
 * - rpll_spell_auras_hooks
 * - rpll_threat_manager_hooks
 * - rpll_combat_manager_hooks
 * 
 * In the header files you can find some notes that describe if the implementation depends on the order.
 * Please follow it patiently, because some hooks in the wrong order may very well cause crashes.
 * 
 * However, you can also choose not to use these hooks and place at all necessary places in your code
 * calls to the functions specified in here.
 * 
 * I recommend to use the hooks though, if you can.
 * If you have any questions, dont bother to contact me directly!
 * 
 * Note: It uses ZMQ, so make sure to install libzmq on your system
 * Most package manger ship it with the package "zeromq"
 */
class RPLLHooks
{
    static uint8_t API_VERSION;

    static std::unordered_map<uint64_t, RPLL_LastUpdate> LAST_UPDATE;
    static std::unordered_map<uint64_t, double> LAST_UNIT_POSITION;
    static float UPDATE_POSITION_LEEWAY;
    static float UPDATE_POSITION_LEEWAY_NPC;
    static float UPDATE_POSITION_LEEWAY_ARENA;

    static uint64_t RAID_IN_BOSS_FIGHT_UPDATE_TIMEOUT;
    static uint64_t RAID_UPDATE_TIMEOUT;
    static uint64_t ARENA_UPDATE_TIMEOUT;
    static uint64_t BATTLEGROUND_UPDATE_TIMEOUT;

    static uint64_t MESSAGE_COUNT;

    // ZMQ Messaging
    static void *zmqContext;
    static void *zmqSocket;
    static void *GetZmqSocket();
    static void SendZmqMessage(ByteBuffer &&msg);

    // Time tracking
    static uint8_t GetCurrentTimeSize();
    static uint64_t GetCurrentTime();

    // Conditions
    static std::set<uint32_t> raidMapIds;
    static bool IsInRaid(Unit *unit);
    static std::set<uint32_t> arenaMapIds;
    static bool IsInArena(Unit *unit);
    static std::set<uint32_t> battlegroundMapIds;
    static bool IsInBattleground(Unit *unit);
    static bool IsInInstance(Unit *unit);

    // Map meta data
    static void AppendMapMetaData(ByteBuffer &msg, Unit *unit);
    static uint8_t GetMapMetaDataSize();

    // Message Meta data
    static void AppendMessageMetaData(ByteBuffer &msg, uint8_t msgType, uint8_t msgLength);
    static uint8_t GetMessageMetaDataSize();

    // Helper
    static void AppendRPLLDamage(ByteBuffer &msg, RPLL_Damage &damage);
    static void Instance(uint32_t mapId, uint32_t instanceId, RPLL_MessageType messageType);

    // Optimization
    static inline bool IsPowerWithinTimeout(Unit *unit, RPLL_PowerType power);
    static inline bool IsPositionWithinTimeout(Unit *unit);
    static inline bool HasSignificantPositionChange(Unit *unit, float x, float y, float z, float orientation);

public:
    // Helper
    static RPLL_Damage BuildRPLLDamage(RPLL_DamageSchool damageSchool, uint32_t damage, uint32_t resisted_or_glanced, uint32_t absorbed);

    // Mapper
    static RPLL_DamageHitType mapHitMaskToRPLLHitType(uint32_t hitMask);
    static RPLL_PowerType mapPowersToRPLLPowerType(Powers power);
    static RPLL_DamageSchool mapSpellSchoolToRPLLDamageSchool(SpellSchools school);
    static RPLL_DamageSchool mapSpellSchoolMaskToRPLLDamageSchool(uint32_t schoolMask);
    static RPLL_PvP_Winner mapPvPWinnerToRPLLPvPWinner(uint8_t winner);

    /* Actual Methods to pack the messages */
    // These are used from the hooks
    // But if you want to include additional information,
    // then you can use these as well
    static void DealMeleeDamage(Unit *attacker, Unit *victim, RPLL_DamageHitType damageHitType, uint32_t blocked, std::vector<RPLL_Damage> &&damages);
    static void DealMeleeDamage(Unit *attacker, Unit *victim, RPLL_DamageHitType damageHitType, uint32_t blocked, RPLL_Damage &&damages);
    static void DealSpellDamage(Unit *attacker, Unit *victim, uint32_t spellId, uint32_t blocked, RPLL_Damage &&damage);
    static void Heal(Unit *caster, Unit *target, uint32_t spellId, uint32_t totalHeal, uint32_t effectiveHeal, uint32_t absorb);
    static void Death(Unit *cause, Unit *victim);
    static void AuraApplication(Unit *caster, Unit *target, uint32_t spellId, uint32_t stackAmount, bool applied);
    static void Dispel(Unit *dispeller, Unit *target, ObjectGuid auraCaster, uint32_t dispelSpellId, uint32_t dispelledSpellId, uint8_t dispelAmount);
    static void SpellSteal(Unit *dispeller, Unit *target, ObjectGuid auraCaster, uint32_t stealSpellId, uint32_t stolenSpellId, uint8_t stealAmount);
    // We just know that we were interrupted, but we need to infer from what in the post analysis
    static void Interrupt(Unit *target, uint32_t interruptedSpellId);
    static void Position(Unit *unit, float x, float y, float z, float orientation);
    static void CombatState(Unit *unit, bool inCombat);
    static void Power(Unit *unit, RPLL_PowerType powerType, uint32_t maxPower, uint32_t currentPower);
    static void StartBattleground(uint32_t mapId, uint32_t instanceId);
    static void StartUnratedArena(uint32_t mapId, uint32_t instanceId);
    static void StartRatedArena(uint32_t mapId, uint32_t instanceId, uint32_t teamId1, uint32_t teamId2);
    static void EndUnratedArena(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner);
    static void EndBattleground(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t scoreAlliance, uint32_t scoreHorde);
    static void EndRatedArena(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t teamId1, uint32_t teamId2, int32_t teamChange1, int32_t teamChange2);
    static void Loot(Unit *unit, uint32_t itemId, uint32_t count);
    static void SpellCast(Unit *caster, uint64_t targetGUID, uint32_t spellId, RPLL_DamageHitType hitType);
    // Amount is an integer which represents the decimal amount of threat.
    // The precision factor is 10
    // E.g. If you did 10.52 threat to a target the amount transmitted is 105
    static void Threat(Unit *threater, Unit *threatened, uint32_t spellId, int32_t amount);
    // See RPLL_Event
    static void Event(Unit *unit, RPLL_Event event);
    static void Summon(Unit *unit, uint64_t ownerGUID);
    static void InstanceDelete(uint32_t instanceId);
};

#endif