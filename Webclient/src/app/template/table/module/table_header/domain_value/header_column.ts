export interface HeaderColumn {
    index: number,
    filter_name: string,
    labelKey: string,
    // 0 => Text
    // 1 => Number
    // 2 => Date
    // 3 => Select
    type: number,
    type_range: string[] | undefined
}
