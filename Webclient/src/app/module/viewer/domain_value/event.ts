import {Unit} from "./unit";
import {SpellComponent} from "./damage";

export type Event = SpellCast | Death | CombatState | Loot | Position | Power | AuraApplication
    | Interrupt | UnAura | ThreatWipe | Summon | MeleeDamage | SpellDamage | Heal | Threat;

export type SpellCast = [number, number, Unit | null, Unit, number, number, number];
export type Death = [number, number, Unit, Unit | undefined];
export type CombatState = [number, number, Unit, boolean];
export type Loot = [number, number, Unit, number, number];
export type Position = [number, number, Unit, number, number, number, number];
export type Power = [number, number, Unit, number, number, number];
export type AuraApplication = [number, number, Unit, Unit, number, number, number];
export type Interrupt = [number, number, number, Unit, Unit, number, number];
export type UnAura = [number, number, number, number, Unit, Unit, number, number];
export type ThreatWipe = [number, number, Unit];
export type Summon = [number, number, Unit, Unit];
export type MeleeDamage = [number, number, Unit, Unit, number, Array<SpellComponent>];
export type SpellDamage = [number, number, number, Unit, Unit, number, number, Array<SpellComponent>];
export type Heal = [number, number, number, Unit, Unit, number, number, number, number, number, number, number, number];
export type Threat = [number, number, number, Unit, Unit, number, number, number, number];
