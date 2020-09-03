export type Unit = [number, number, number | null, number | null];

export function get_unit_id(unit: Unit): number {
    if (!unit)
        return 0;
    return unit[1];
}

export function get_creature_entry(unit: Unit): number | null {
    return unit[2];
}

export function is_player(unit: Unit): boolean {
    return unit[0] === 1;
}

export function is_creature(unit: Unit): boolean {
    return unit[0] === 0;
}

export function has_unit(container: Array<Unit>, unit: Unit): boolean {
    return container.find(inner_unit => unit[0] === inner_unit[0] && unit[1] === inner_unit[1]) !== undefined;
}
