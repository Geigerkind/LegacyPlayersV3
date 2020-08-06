#include "rpll_threat_manager_hooks.h"

void RPLLThreatManagerHooks::AddThreat(const Unit* owner, const Unit* target, SpellInfo const* spell, const float amountBefore, const float amountAfter) {
    if (owner == nullptr || target == nullptr || std::fabs(amountAfter-amountBefore) <= 0.01f)
        return;
    RPLLHooks::Threat(target, owner, spell == nullptr ? 0 : spell->Id, static_cast<int32_t>((amountAfter-amountBefore)*10));
}

void RPLLThreatManagerHooks::ScaleThreat(const Unit* owner, const Unit* target, const float factor) {
    if (owner == nullptr || target == nullptr)
        return;
    if (factor <= 0.01f && owner->GetGUID().IsCreature()
        && (owner->ToCreature()->IsWorldBoss() || owner->ToCreature()->IsDungeonBoss())
    ) {
        RPLLHooks::Event(target, RPLL_Event::RPLL_THREAT_WIPE);
    }
}