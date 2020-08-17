import {UtilService} from "../service/util";
import {BehaviorSubject} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {get_unit_id, Unit} from "../../../domain_value/unit";
import {group_by} from "../../../../../stdlib/group_by";
import {get_aura_application, get_dispel, get_spell_cast, get_spell_cause} from "../../../extractor/events";
import {UnAuraOverviewRow} from "../module/un_aura_overview/domain_value/un_aura_overview_row";
import {se_dispel} from "../../../extractor/sources";

function commit_dispel(utilService: UtilService, data$: BehaviorSubject<Array<[number, Array<UnAuraOverviewRow>]>>,
                       units$: Map<number, RaidMeterSubject>, spell_casts: Map<number, Event>, aura_applications: Map<number, Event>,
                       dispels: Array<Event>, dispel_unit_extraction: (Event) => Unit): void {
    const newData = new Map();

    const grouping = group_by(dispels, (event) => get_unit_id(dispel_unit_extraction(event)));
    // tslint:disable-next-line:forin
    for (const unit_id in grouping) {
        const subject_id = Number(unit_id);
        if (!units$.has(subject_id))
            units$.set(subject_id, utilService.get_row_unit_subject(dispel_unit_extraction(grouping[unit_id][0])));
        if (!newData.has(subject_id))
            newData.set(subject_id, []);
        const un_aura_overview_row = newData.get(subject_id);
        grouping[subject_id].forEach(event => {
            const dispel_event = get_dispel(event);
            const [indicator, spell_cause_event] = get_spell_cause(dispel_event.cause_event_id, spell_casts, aura_applications);
            const spell_cause = indicator ? get_spell_cast(spell_cause_event) : get_aura_application(spell_cause_event);
            const dispelled_aura_event = aura_applications?.get(dispel_event.target_event_id);
            if (!!spell_cause && !!dispelled_aura_event) {
                un_aura_overview_row.push({
                    timestamp: event.timestamp,
                    caster: se_dispel(spell_casts, aura_applications)(event),
                    target: dispelled_aura_event.subject,
                    caster_spell_id: (spell_cause as any).spell_id,
                    target_spell_id: get_aura_application(dispelled_aura_event).spell_id
                });
            }
        });
    }

    // @ts-ignore
    data$.next([...newData.entries()]);
}

export {commit_dispel};
