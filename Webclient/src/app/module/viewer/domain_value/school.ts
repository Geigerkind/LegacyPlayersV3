export enum School {
    Physical = "Physical",
    Holy = "Holy",
    Fire = "Fire",
    Nature = "Nature",
    Frost = "Frost",
    Shadow = "Shadow",
    Arcane = "Arcane"
}

export function get_all_schools(): Array<School> {
    return [School.Physical, School.Holy, School.Fire, School.Nature,
        School.Frost, School.Shadow, School.Arcane];
}
