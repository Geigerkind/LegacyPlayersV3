import {Player} from "./player";
import {Creature} from "./creature";

export type Unit = Player | Creature | [boolean, number, number | null];

function get_unit_id(unit: Unit): number {
    if (!unit)
        return 0;
    if (is_player(unit))
        return !!(unit as any).length ? unit[1] : ((unit as any).Player as Player).character_id;
    if (is_creature(unit))
        return !!(unit as any).length ? unit[1] : ((unit as any).Creature as Creature).creature_id;
    return unit[1];
}

export function get_creature_entry(unit: Unit): number {
    return !!(unit as any).length ? unit[2] : ((unit as any).Creature as Creature).entry;
}

function is_player(unit: Unit): boolean {
    return !!(unit as any).Player || (!!(unit as any).length && unit[0] === 1);
}

function is_creature(unit: Unit): boolean {
    return !!(unit as any).Creature || (!!(unit as any).length && unit[0] === 0);
}

function has_unit(container: Array<Unit>, unit: Unit): boolean {
    return container.find(inner_unit => ((is_player(unit) && is_player(inner_unit)) || (is_creature(unit) && is_creature(inner_unit)))
        && get_unit_id(unit) === get_unit_id(inner_unit)) !== undefined;
}

export {get_unit_id, is_creature, is_player, has_unit};
