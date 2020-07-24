import {Player} from "./player";
import {Creature} from "./creature";

export type Unit = Player | Creature;

function get_unit_id(unit: Unit): number {
    if (!!unit.Player)
        return (unit.Player as Player).character_id;
    if (!!unit.Creature)
        return (unit.Creature as Creature).creature_id;
    return 0;
}

export { get_unit_id };
