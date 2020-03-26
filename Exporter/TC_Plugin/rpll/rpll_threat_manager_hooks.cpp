#include "rpll_threat_manager_hooks.h"

void RPLLThreatManagerHooks::AddThreat(Unit* owner, Unit* target, SpellInfo const* spell, float amountBefore, float amountAfter) {
    if (owner == nullptr || target == nullptr || std::fabs(amountAfter-amountBefore) <= 0.01f)
        return;
    RPLLHooks::Threat(target, owner, spell == nullptr ? 0 : spell->Id, static_cast<int32_t>((amountAfter-amountBefore)*10));
}

void RPLLThreatManagerHooks::ScaleThreat(Unit* owner, Unit* target, float factor) {
    if (owner == nullptr || target == nullptr)
        return;
    if (factor <= 0.01f && owner->GetGUID().IsCreature()
        && (owner->ToCreature()->IsWorldBoss() || owner->ToCreature()->IsDungeonBoss())
    ) {
        RPLLHooks::Event(target, RPLL_Event::RPLL_THREAT_WIPE);
    }
}