import {Player} from "./player";
import {Creature} from "./creature";

export type Unit = Player | Creature;

function get_unit_id(unit: Unit): number {
    if (is_player(unit))
        return ((unit as any).Player as Player).character_id;
    if (is_creature(unit))
        return ((unit as any).Creature as Creature).creature_id;
    return 0;
}

function is_player(unit: Unit): boolean {
    return !!(unit as any).Player;
}

function is_creature(unit: Unit): boolean {
    return !!(unit as any).Creature;
}

export { get_unit_id, is_creature, is_player };
