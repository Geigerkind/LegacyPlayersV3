import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {Event} from "../../../domain_value/event";
import {
    get_aura_application, get_death, get_melee_damage,
    get_spell_cast,
    get_spell_cause,
    get_spell_damage
} from "../../../extractor/events";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {UtilService} from "../service/util";
import {BehaviorSubject} from "rxjs";
import {CONST_UNKNOWN_ABILITY_ID} from "../../../constant/viewer";
import {DeathOverviewRow} from "../module/deaths_overview/domain_value/death_overview_row";

function commit_death(utilService: UtilService, data$: BehaviorSubject<Array<[number, Array<DeathOverviewRow>]>>,
                      units$: Map<number, RaidMeterSubject>, melee_damage: Map<number, Event>, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>,
                      spell_damage: Array<Event>, deaths: Array<Event>, melee_unit_extraction: (Event) => Unit,
                      spell_unit_extraction: (Event) => Unit, death_unit_extraction: (Event) => Unit): void {
    const newData = new Map();

    const grouping = group_by(deaths, (event) => get_unit_id(death_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!units$.has(subject_id))
            units$.set(subject_id, utilService.get_row_unit_subject(death_unit_extraction(grouping[unit_id][0])));
        if (!newData.has(subject_id))
            newData.set(subject_id, []);

        const death_overview_rows = newData.get(subject_id);
        grouping[subject_id].forEach(event => {
            // For each death find the ability that executed this death
            let spell_id;
            let found_spell_min_ts = Number.MAX_VALUE;
            let amount = 0;
            for (const [md_index, md_event] of melee_damage) {
                const difference = Math.abs(md_event.timestamp - event.timestamp);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(melee_unit_extraction(md_event)) === subject_id && difference < found_spell_min_ts) {
                    const kb_amount = get_melee_damage(md_event).damage;
                    if (kb_amount > 0) {
                        spell_id = 0;
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }
            // @ts-ignore
            for (const [sd_event_id, sd_event] of spell_damage.entries()) {
                const difference = Math.abs(sd_event.timestamp - event.timestamp);
                if (difference >= 1000)
                    continue;
                if (get_unit_id(spell_unit_extraction(sd_event)) === subject_id && difference < found_spell_min_ts) {
                    const [indicator, cause_event] = get_spell_cause(get_spell_damage(sd_event).spell_cause_id, spell_casts, aura_applications);
                    const kb_amount = get_spell_damage(sd_event).damage.damage;
                    if (kb_amount > 0) {
                        spell_id = indicator ? get_spell_cast(cause_event).spell_id : get_aura_application(cause_event).spell_id;
                        found_spell_min_ts = difference;
                        amount = kb_amount;
                    }
                    break;
                }
            }
            if (spell_id === undefined)
                spell_id = CONST_UNKNOWN_ABILITY_ID;

            const death_event = get_death(event);
            death_overview_rows.push({
                timestamp: event.timestamp,
                murder: death_event.murder,
                murdered: event.subject,
                killing_blow: {
                    ability_id: spell_id,
                    amount
                }
            });
        });
    }

    // @ts-ignore
    data$.next([...newData.entries()]);
}

export {commit_death};
