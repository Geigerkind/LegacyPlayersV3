export interface ItemTooltip {
    name: string,
    icon: string,
    quality: number,
    inventory_type: string | undefined,
    bonding: string | undefined,
    sheath_type: string | undefined,
    sub_class: string,
    armor: number | undefined,
    stats: { value: number, name: string }[] | undefined,
    durability: number | undefined,
    item_level: number | undefined,
    required_level: number | undefined,
    item_effects: string[] | undefined,
    item_set: { name: string, set_items: { item_id: number, active: boolean, name: string }[], set_effects: { threshold: number, description: string }[] } | undefined,
    socket: { socket_bonus: string, slots: { flag: number, item: { icon: string, effect: string, flag: number } | undefined }[] } | undefined,
    enchant: string | undefined,
    weapon_stat: { delay: number, damage_sources: { damage_min: number, damage_max: number, damage_type: string | undefined }[] } | undefined
}
