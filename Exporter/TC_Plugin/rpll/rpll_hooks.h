#ifndef _RPLL_HOOKS_H
#define _RPLL_HOOKS_H

#include <zmq.h>
#include <cinttypes>
// Causes cyclic dependency but this thing provides
// all needed data structures and uses nothing else
// so, so be it
#include "Unit.h"
#include "ByteBuffer.h"
#include "SpellAuraEffects.h"
#include "SpellAuras.h"
#include "Spell.h"
#include "Battlegrounds/Battleground.h"
#include "Item.h"
#include "Player.h"

enum RPLL_MessageType : uint8_t {
    RPLL_MSG_MELEE_DAMAGE = 0, // Works
    RPLL_MSG_SPELL_DAMAGE = 1, // Works
    RPLL_MSG_HEAL = 2, // Works
    RPLL_MSG_DEATH = 3, // Works
    RPLL_MSG_AURA_APPLICATION = 4, // Works
    RPLL_MSG_DISPEL = 5, // Works
    RPLL_MSG_SPELL_STEAL = 6, // Works, but needs post analysis
    RPLL_MSG_INTERRUPT = 7, // Works, but needs post analysis
    RPLL_MSG_POSITION = 8, // Works
    RPLL_MSG_COMBAT_STATE = 9, // Works
    RPLL_MSG_POWER = 10, // Works
    RPLL_MSG_LOOT = 11, // Works
    RPLL_MSG_SPELL_CAST = 12, // Works
    RPLL_MSG_THREAT = 13, // Works
    RPLL_MSG_EVENT = 14, // See events
    RPLL_MSG_SUMMON = 15, // Works
    RPLL_MSG_INSTANCE_PVP_START = 16, // Works
    RPLL_MSG_INSTANCE_PVP_END_UNRATED_ARENA = 17, // Works
    RPLL_MSG_INSTANCE_PVP_END_RATED_ARENA = 18, // Works
    RPLL_MSG_INSTANCE_PVP_END_BATTLEGROUND = 19 // Works
};

enum RPLL_Event : uint8_t {
    RPLL_THREAT_WIPE // Does not work!
};

enum RPLL_DamageHitType : uint8_t {
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

enum RPLL_DamageSchool : uint8_t {
    RPLL_PHYSICAL = 0,
    RPLL_HOLY = 1,
    RPLL_FIRE = 2,
    RPLL_NATURE = 3,
    RPLL_FROST = 4,
    RPLL_SHADOW = 5,
    RPLL_ARCANE = 6,
    RPLL_DS_UNDEFINED = 255
};

enum RPLL_PowerType : uint8_t {
    RPLL_MANA = 0,
    RPLL_RAGE = 1,
    RPLL_FOCUS = 2,
    RPLL_ENERGY = 3,
    RPLL_HAPPINESS = 4,
    RPLL_HEALTH = 5,
    RPLL_PWT_UNDEFINED = 255
};

enum RPLL_PvP_Winner : uint8_t {
    RPLL_TEAM_NONE = 0,
    RPLL_TEAM_HORDE = 1,
    RPLL_TEAM_ALLIANCE = 2
};

struct RPLL_Damage {
    RPLL_DamageSchool damageSchool;
    uint32_t damage;
    uint32_t resisted_or_glanced;
    uint32_t absorbed;
};

/*
 * HOW TO INSTALL
 * Below you will find some public methods
 * 1. The actualy methods to send packages
 * 2. One example of hooks that sufficiently collects all needed information
 * 
 * Above the hooks some notes are described if the implementation depends on the order.
 * Please follow it patiently, because some hooks in the wrong order may very well cause crashes.
 * 
 * However, you can also choose not to use these hooks methods and place at all necessary places in your code
 * call to the method sending functions.
 * 
 * I recommend to use the hooks though.
 * If you have any questions, dont bother to contact me directly!
 * 
 * Note: It uses ZMQ, so make sure to install libzmq on your system
 * Most package manger ship it with the package "zeromq"
 */
class RPLLHooks
{
    static uint8_t API_VERSION;
    static bool DEBUG;
    static void PrintDebugMessage(const char* msg, bool overrule = false);

    static std::map<uint64_t, double> LAST_UNIT_POSITION;
    static float UPDATE_POSITION_LEEWAY;
    static float UPDATE_POSITION_LEEWAY_ARENA;

    static std::map<uint64_t, uint64_t> LAST_HEALTH_UPDATE;
    static std::map<uint64_t, uint64_t> LAST_POWER_UPDATE;
    static std::map<uint64_t, uint64_t> LAST_POSITION_UPDATE;

    static uint64_t RAID_IN_BOSS_FIGHT_UPDATE_TIMEOUT;
    static uint64_t RAID_UPDATE_TIMEOUT;
    static uint64_t ARENA_UPDATE_TIMEOUT;
    static uint64_t BATTLEGROUND_UPDATE_TIMEOUT;

    // ZMQ Messaging
    static void* zmqContext;
    static void* zmqSocket;
    static void* GetZmqSocket();
    static void SendZmqMessage(ByteBuffer &&msg);
    
    // Time tracking
    static uint8_t GetCurrentTimeSize();
    static uint64_t GetCurrentTime();

    // Conditions
    static std::set<uint32_t> raidMapIds;
    static bool IsInRaid(Unit* unit);
    static std::set<uint32_t> arenaMapIds;
    static bool IsInArena(Unit* unit);
    static std::set<uint32_t> battlegroundMapIds;
    static bool IsInBattleground(Unit* unit);
    static bool IsInInstance(Unit* unit);

    // Map meta data
    static void AppendMapMetaData(ByteBuffer &msg, Unit* unit);
    static uint8_t GetMapMetaDataSize();

    // Message Meta data
    static void AppendMessageMetaData(ByteBuffer &msg, uint8_t msgType, uint8_t msgLength);
    static uint8_t GetMessageMetaDataSize();

    // Helper
    static RPLL_Damage BuildRPLLDamage(RPLL_DamageSchool damageSchool, uint32_t damage, uint32_t resisted_or_glanced, uint32_t absorbed);
    static void AppendRPLLDamage(ByteBuffer &msg, RPLL_Damage &damage);

    // Mapper
    static RPLL_DamageHitType mapHitMaskToRPLLHitType(uint32_t hitMask);
    static RPLL_PowerType mapPowersToRPLLPowerType(Powers power);
    static RPLL_DamageSchool mapSpellSchoolToRPLLDamageSchool(SpellSchools school);
    static RPLL_DamageSchool mapSpellSchoolMaskToRPLLDamageSchool(uint32_t schoolMask);
    static RPLL_PvP_Winner mapPvPWinnerToRPLLPvPWinner(uint8_t winner);

    // Optimization
    static inline bool IsPowerWithinTimeout(Unit* unit, RPLL_PowerType power);
    static inline bool IsPositionWithinTimeout(Unit* unit);
    static inline bool HasSignificantPositionChange(Unit* unit, float x, float y, float z, float orientation);

    // Debugging
    static void PrintMessage(ByteBuffer &&msg);

public:
    /* Actual Methods to pack the messages */
    // These are used from the hooks
    // But if you want to include additional information,
    // then you can use these as well
    static void DealMeleeDamage(Unit* attacker, Unit* victim, RPLL_DamageHitType damageHitType, uint32_t blocked, std::vector<RPLL_Damage>&& damages);
    static void DealMeleeDamage(Unit* attacker, Unit* victim, RPLL_DamageHitType damageHitType, uint32_t blocked, RPLL_Damage&& damages);
    static void DealSpellDamage(Unit* attacker, Unit* victim, uint32_t spellId, uint32_t blocked, RPLL_Damage&& damage);
    static void Heal(Unit* caster, Unit* target, uint32_t spellId, uint32_t totalHeal, uint32_t effectiveHeal, uint32_t absorb);
    static void Death(Unit* cause, Unit* victim);
    static void AuraApplication(Unit* caster, Unit* target, uint32_t spellId, uint32_t stackAmount, bool applied);
    static void Dispel(Unit* dispeller, Unit* target, ObjectGuid auraCaster, uint32_t dispelSpellId, uint32_t dispelledSpellId, uint8_t dispelAmount);
    static void SpellSteal(Unit* dispeller, Unit* target, ObjectGuid auraCaster, uint32_t stealSpellId, uint32_t stolenSpellId, uint8_t stealAmount);
    // We just know that we were interrupted, but we need to infer from what in the post analysis
    static void Interrupt(Unit* target, uint32_t interruptedSpellId);
    static void Position(Unit* unit, float x, float y, float z, float orientation);
    static void CombatState(Unit* unit, bool inCombat);
    static void Power(Unit * unit, RPLL_PowerType powerType, uint32_t maxPower, uint32_t currentPower);
    static void Instance(uint32_t mapId, uint32_t instanceId);
    static void Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner);
    static void Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t scoreAlliance, uint32_t scoreHorde);
    static void Instance(uint32_t mapId, uint32_t instanceId, RPLL_PvP_Winner winner, uint32_t teamId1, uint32_t teamId2, int32_t teamChange1, int32_t teamChange2);
    static void Loot(Unit* unit, uint32_t itemId);
    static void SpellCast(Unit* caster, uint64_t targetGUID, uint32_t spellId, RPLL_DamageHitType hitType);
    // Amount is an integer which represents the decimal amount of threat.
    // The precision factor is 10
    // E.g. If you did 10.52 threat to a target the amount transmitted is 105
    static void Threat(Unit* threater, Unit* threatened, uint32_t spellId, int32_t amount);
    // See RPLL_Event
    static void Event(Unit* unit, RPLL_Event event);
    static void Summon(Unit* unit, uint64_t ownerGUID);

    /* Hook methods from Unit */
    static void SendAttackStateUpdate(CalcDamageInfo *damageInfo);
    static void SendSpellNonMeleeDamageLog(SpellNonMeleeDamage *damageInfo);
    static void SendPeriodicAuraLog(SpellPeriodicAuraLogInfo* pInfo);
    // IMPORTANT: Add hook AFTER original function
    static void DealHeal(HealInfo& healInfo);
    static void Kill(Unit* attacker, Unit* victim);
    // Note: Target is the Unit on whom it is called, i.e. "this"
    static void RemoveAurasDueToSpellByDispel(Unit* target, uint32_t spellId, uint32_t dispellerSpellId, ObjectGuid casterGUID, WorldObject* dispeller, uint8 chargesRemoved = 1);
    // Note: The implementation may not give you the used spell to steal
    // Set the stealSpellId to 0 in this case
    // The target is again the Unit, i.e. "this"
    static void RemoveAurasDueToSpellBySteal(Unit* target, uint32_t spellId, uint32_t stealSpellId, ObjectGuid casterGUID, WorldObject* stealer);
    // Note: Only call if the position update was successful
    static void UpdatePosition(Unit* unit, float x, float y, float z, float orientation);
    // IMPORTANT: These have to be called AFTER the original function.
    // BUT save the old value before!
    static void SetHealth(Unit* unit, uint32_t oldVal);
    static void SetMaxHealth(Unit* unit, uint32_t oldVal);
    static void SetPower(Unit* unit, Powers powerType, uint32_t oldVal);
    static void SetMaxPower(Unit* unit, Powers powerType, uint32_t oldVal);
    // IMPORTANT: Make sure to call it BEFORE the original call
    static void RemoveOwnedAura(Aura* aura);
    // Note: Unit is "this"
    static void SetOwnerGUID(Unit* unit, ObjectGuid owner);

    /* Hook methods for CombatManager */
    static void UpdateOwnerCombatState(Unit* unit, bool result);

    /* Hook Methods for Battleground */
    static void StartBattleground(Battleground* battleground);
    static void EndBattleground(Battleground* battleground, uint32_t *scores);

    /* Hook Methods for Player */
    static void SendNewItem(Unit* unit, Item *item, uint32_t count, bool received, bool created, bool broadcast, bool sendChatMessage);
    // Note: Use the player for the unit
    static void EnvironmentalDamage(Unit* unit, EnviromentalDamage type, uint32_t damage, uint32_t result);
    // Used to get state on login/logout instance enter etc.
    static void SetMap(Unit* unit);

    /* Hook Methods for Spell */
    static void SendCastResult(const Spell* spell, SpellCastResult result);
    // IMPORTANT: Call after the original function
    static void DoDamageAndTriggers(const Spell* spell, uint32 hitMask);

    /* Hook Methods for SpellAura */
    static void AuraCreate(Aura* result);
    // IMPORTANT: Save the amount here BEFORE calling the original function
    static void AuraSetStackAmount(Aura* aura, uint32_t oldAmount);

    /* Hook Methods for the ThreatManager */
    // Notes:
    // Owner => _owner or GetOwner()
    // AmountBefore => GetThreat of the target called BEFORE calling the original AddThreat function
    // AmountAfter => GetThreat of the target called AFTER calling the original AddThreat function
    static void AddThreat(Unit* owner, Unit* target, SpellInfo const* spell, float amountBefore, float amountAfter);
    static void ScaleThreat(Unit* owner, Unit* target, float factor);
};

#endif