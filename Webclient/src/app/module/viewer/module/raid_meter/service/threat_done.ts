import {Injectable, OnDestroy} from "@angular/core";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {RaidMeterSubject} from "../../../../../template/meter_graph/domain_value/raid_meter_subject";
import {Event} from "../../../domain_value/event";
import {ChangedSubject, InstanceDataService} from "../../../service/instance_data";
import {UtilService} from "./util";
import {take} from "rxjs/operators";
import {ce_threat} from "../../../extractor/causes";
import {group_by} from "../../../../../stdlib/group_by";
import {get_unit_id} from "../../../domain_value/unit";
import {CONST_AUTO_ATTACK_ID} from "../../../constant/viewer";
import {get_threat} from "../../../extractor/events";
import {ae_spell_cast_or_aura_application} from "../../../extractor/abilities";

@Injectable({
    providedIn: "root",
})
export class ThreatDoneService implements OnDestroy {

    private subscription_changed: Subscription;

    private data$: BehaviorSubject<Array<[number, Array<[number, number]>]>> = new BehaviorSubject([]);
    private abilities$: Map<number, RaidMeterSubject>;
    private units$: Map<number, RaidMeterSubject>;

    private initialized: boolean = false;

    private aura_applications: Map<number, Event> = new Map();
    private spell_casts: Map<number, Event> = new Map();
    private threat: Array<Event> = [];

    constructor(
        private instanceDataService: InstanceDataService,
        private utilService: UtilService
    ) {
    }

    ngOnDestroy(): void {
        this.subscription_changed?.unsubscribe();
    }

    get_data(abilities: Map<number, RaidMeterSubject>, units: Map<number, RaidMeterSubject>): Observable<Array<[number, Array<[number, number]>]>> {
        if (!this.initialized) {
            this.abilities$ = abilities;
            this.units$ = units;
            this.initialize();
        }
        return this.data$.asObservable();
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
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ChangedSubject.Attempts, ChangedSubject.Threat].includes(changed))
                this.instanceDataService.get_threat().pipe(take(1)).subscribe(threat => {
                    this.threat = threat;
                    this.commit();
                });
        });
    }

    private commit(): void {
        const newData = new Map();

        const grouping = group_by(this.threat, (event) => get_unit_id(event.subject));
        // tslint:disable-next-line:forin
        for (const unit_id in grouping) {
            const subject_id = Number(unit_id);
            if (!this.units$.has(subject_id))
                this.units$.set(subject_id, this.utilService.get_row_unit_subject(grouping[unit_id][0].subject));
            if (!newData.has(subject_id))
                newData.set(subject_id, new Map());

            const abilities_data = newData.get(subject_id);
            grouping[subject_id].forEach(event => {
                let spell_id = ae_spell_cast_or_aura_application(ce_threat, this.spell_casts, this.aura_applications)(event)[0];
                if (!spell_id)
                    spell_id = CONST_AUTO_ATTACK_ID;
                const threat = get_threat(event).threat.amount;
                if (!this.abilities$.has(spell_id))
                    this.abilities$.set(spell_id, this.utilService.get_row_ability_subject(spell_id));

                if (abilities_data.has(spell_id)) abilities_data.set(spell_id, abilities_data.get(spell_id) + threat);
                else abilities_data.set(spell_id, threat);
            });
        }

        // @ts-ignore
        this.data$.next([...newData.entries()]
            .map(([unit_id, abilities]) => [unit_id, [...abilities.entries()]]));
    }
}
