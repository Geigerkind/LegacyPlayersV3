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
    Deaths = "Deaths"
}

function is_event_data_set(data_set: DataSet): boolean {
    return [DataSet.Deaths].includes(data_set);
}

function get_point_style(data_set: DataSet): PointStyle | HTMLImageElement {
    switch (data_set) {
        case DataSet.Deaths:
            const death_icon = new Image();
            death_icon.src = "/assets/viewer/death_icon.png";
            return death_icon;
    }
    return "circle";
}

export {is_event_data_set, get_point_style};
