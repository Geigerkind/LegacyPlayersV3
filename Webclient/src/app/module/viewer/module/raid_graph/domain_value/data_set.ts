import {PointStyle} from "chart.js";

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

    // Events
    Deaths = "Deaths",
    Kills = "Kills",
    DispelsDone = "Dispels done",
    DispelsReceived = "Dispels received",
}

function is_event_data_set(data_set: DataSet): boolean {
    return [DataSet.Deaths, DataSet.Kills, DataSet.DispelsDone, DataSet.DispelsReceived].includes(data_set);
}

function get_point_style(data_set: DataSet): PointStyle | HTMLImageElement {
    switch (data_set) {
        case DataSet.Deaths: {
            const death_icon = new Image();
            death_icon.src = "/assets/viewer/death_icon.png";
            return death_icon;
        }
        case DataSet.Kills: {
            const kill_icon = new Image();
            kill_icon.src = "/assets/viewer/attack_icon.png";
            kill_icon.width = 20;
            kill_icon.height = 20;
            return kill_icon;
        }
        case DataSet.DispelsDone:
        case DataSet.DispelsReceived: {
            const dispel_icon = new Image();
            dispel_icon.src = "/assets/viewer/dispel_icon.png";
            dispel_icon.width = 20;
            dispel_icon.height = 20;
            return dispel_icon;
        }
    }
    return "circle";
}

export {is_event_data_set, get_point_style};
