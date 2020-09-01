export type SpellComponent = [number, number, number, number, number];

export function get_spell_components_total_amount(components: Array<SpellComponent>): number {
    return components.reduce((acc, component) => acc + component[0], 0);
}
