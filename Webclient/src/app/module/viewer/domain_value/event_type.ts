import {SpellCast} from "./spell_cast";
import {Death} from "./death";
import {CombatState} from "./combat_state";
import {Loot} from "./loot";
import {Position} from "./position";
import {Power} from "./power";
import {AuraApplication} from "./aura_application";
import {Interrupt} from "./interrupt";
import {SpellSteal} from "./spell_steal";
import {ThreatWipe} from "./threat_wipe";
import {Summon} from "./summon";
import {MeleeDamage} from "./melee_damage";
import {SpellDamage} from "./spell_damage";
import {Heal} from "./heal";
import {Threat} from "./threat";

export type EventType = SpellCast | Death | CombatState | Loot | Position | Power | AuraApplication | Interrupt | SpellSteal | ThreatWipe | Summon | MeleeDamage | SpellDamage | Heal | Threat;
