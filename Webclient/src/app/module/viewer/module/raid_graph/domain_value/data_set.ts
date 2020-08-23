export enum DataSet {
    DamageDone = "Damage done",
    DamageTaken = "Damage taken",
    TotalHealingDone = "Total healing done",
    TotalHealingTaken = "Total healing taken",
    EffectiveHealingDone = "Effective healing done",
    EffectiveHealingTaken = "Effective healing taken",
    OverhealingDone = "Overhealing done",
    OverhealingTaken = "Overhealing taken",
    ThreatDone = "Threat done",
    ThreatTaken = "Threat taken",

    // Events
    Deaths = "Deaths",
    Kills = "Kills",
    DispelsDone = "Dispels done",
    DispelsReceived = "Dispels received",
    InterruptDone = "Interrupt done",
    InterruptReceived = "Interrupt received",
}

export function is_event_data_set(data_set: DataSet): boolean {
    return [DataSet.Deaths, DataSet.Kills, DataSet.DispelsDone, DataSet.DispelsReceived, DataSet.InterruptDone, DataSet.InterruptReceived].includes(data_set);
}
