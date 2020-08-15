import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {SelectOption} from "../../../../../template/input/select_input/domain_value/select_option";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {SpellService} from "../../../service/spell";
import {map, take} from "rxjs/operators";
import {commit_damage_detail} from "../stdlib/damage_detail";
import {DelayedLabel} from "../../../../../stdlib/delayed_label";
import {CONST_AUTO_ATTACK_ID, CONST_AUTO_ATTACK_LABEL, CONST_UNKNOWN_LABEL} from "../../../constant/viewer";
import {
    get_aura_application,
    get_melee_damage,
    get_spell_cast,
    get_spell_damage,
    get_threat
} from "../../../extractor/events";
import {Damage} from "../../../domain_value/damage";
import {detail_row_post_processing} from "../stdlib/util";
import {create_array_from_nested_map} from "../../../../../stdlib/map_persistance";

@Injectable({
    providedIn: "root",
})
export class ThreatDoneDetailService implements OnDestroy {

    private subscription_changed: Subscription;

    private abilities$: BehaviorSubject<Array<SelectOption>> = new BehaviorSubject([]);
    private ability_details$: BehaviorSubject<Array<[number, Array<[HitType, DetailRow]>]>> = new BehaviorSubject([]);

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private melee_damage: Map<number, Event> = new Map();
    private threat: Array<Event> = [];

    constructor(
        private instanceDataService: InstanceDataService,
        private spellService: SpellService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get ability_and_details(): [Observable<Array<SelectOption>>, Observable<Array<[number, Array<[HitType, DetailRow]>]>>] {
        if (!this.initialized)
            this.initialize();
        return [this.abilities$.asObservable(), this.ability_details$.asObservable()];
    }

    private initialize(): void {
        this.initialized = true;
        this.instanceDataService.get_aura_applications(true).pipe(take(1)).subscribe(aura_applications => {
            this.aura_applications = aura_applications;
            this.commit();
        });
        this.instanceDataService.get_spell_casts().pipe(take(1)).subscribe(spell_casts => {
            this.spell_casts = spell_casts;
            this.commit();
        });
        this.instanceDataService.get_melee_damage().pipe(take(1)).subscribe(melee_damage => {
            this.melee_damage = melee_damage;
            this.commit();
        });
        this.instanceDataService.get_threat().pipe(take(1)).subscribe(threat => {
            this.threat = threat;
            this.commit();
        });

        this.subscription_changed = this.instanceDataService.changed.subscribe(changed => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.AuraApplication].includes(changed))
                this.instanceDataService.get_aura_applications(true).pipe(take(1)).subscribe(aura_applications => {
                    this.aura_applications = aura_applications;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.SpellCast].includes(changed))
                this.instanceDataService.get_spell_casts().pipe(take(1)).subscribe(spell_casts => {
                    this.spell_casts = spell_casts;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.MeleeDamage].includes(changed))
                this.instanceDataService.get_melee_damage().pipe(take(1)).subscribe(melee_damage => {
                    this.melee_damage = melee_damage;
                    this.commit();
                });
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.Threat].includes(changed))
                this.instanceDataService.get_threat().pipe(take(1)).subscribe(threat => {
                    this.threat = threat;
                    this.commit();
                });
        });
    }

    private commit(): void {
        const abilities = new Map<number, DelayedLabel | string>();
        const ability_details = new Map<number, Map<HitType, DetailRow>>();

        for (const event of this.threat) {
            const threat_event = get_threat(event);
            const threat_cause_id = threat_event.cause_event_id;
            const threat_cause_event = this.spell_casts.has(threat_cause_id) ? this.spell_casts.get(threat_cause_id) : this.aura_applications.get(threat_cause_id);

            let hit_type;
            let spell_id;
            if (!threat_cause_event) {
                const melee_damage_event = this.melee_damage.get(threat_cause_id);
                if (!melee_damage_event)
                    return;

                hit_type = get_melee_damage(melee_damage_event).hit_type;
                spell_id = 0;
            } else {
                const spell_cast_event = get_spell_cast(threat_cause_event);
                hit_type = !!spell_cast_event ? spell_cast_event.hit_type : HitType.Hit;
                spell_id = !!spell_cast_event ? spell_cast_event.spell_id : get_aura_application(threat_cause_event).spell_id;
            }

            const threat = threat_event.threat.amount;
            if (!ability_details.has(spell_id)) {
                abilities.set(spell_id, (new DelayedLabel(this.spellService.get_localized_basic_spell(spell_id)
                    .pipe(map(spell => !spell ? CONST_UNKNOWN_LABEL : spell.localization)))));
                ability_details.set(spell_id, new Map());
            }
            const details_map = ability_details.get(spell_id);

            if (details_map.has(hit_type)) {
                const details = details_map.get(hit_type);
                ++details.count;
                details.amount += threat;
                details.min = Math.min(details.min, threat);
                details.max = Math.max(details.max, threat);
            } else {
                details_map.set(hit_type, {
                    amount: threat,
                    amount_percent: 0,
                    average: 0,
                    count: 1,
                    count_percent: 0,
                    hit_type,
                    max: threat,
                    min: threat,
                    glance_or_resist: 0,
                    block: 0,
                    absorb: 0
                });
            }
        }

        detail_row_post_processing(ability_details);

        // @ts-ignore
        this.abilities$.next([...abilities.entries()].map(([value, label_key]) => {
            return {value, label_key};
        }));
        this.ability_details$.next(create_array_from_nested_map(ability_details));
    }
}
