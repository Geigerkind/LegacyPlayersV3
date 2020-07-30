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

function has_unit(container: Array<Unit>, unit: Unit): boolean {
    return container.find(inner_unit => ((is_player(unit) && is_player(inner_unit)) || (is_creature(unit) && is_creature(inner_unit)))
        && get_unit_id(unit) === get_unit_id(inner_unit)) !== undefined;
}

export {get_unit_id, is_creature, is_player, has_unit};
