import {Player} from "./player";
import {Creature} from "./creature";

export type Unit = Player | Creature;

function get_unit_id(unit: Unit): number {
    if (is_player(unit))
        return (unit.Player as Player).character_id;
    if (is_creature(unit))
        return (unit.Creature as Creature).creature_id;
    return 0;
}

function is_player(unit: Unit): boolean {
    return !!unit.Player;
}

function is_creature(unit: Unit): boolean {
    return !!unit.Creature;
}

export { get_unit_id, is_creature, is_player };
