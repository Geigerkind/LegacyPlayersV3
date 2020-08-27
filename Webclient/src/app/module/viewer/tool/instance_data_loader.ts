import {Event} from "../domain_value/event";
import {get_unit_id, Unit} from "../domain_value/unit";
import {se_aura_app_or_own, se_dispel, se_identity, se_interrupt, se_spell_steal} from "../extractor/sources";
import {
    te_aura_application, te_dispel, te_heal, te_melee_damage,
    te_spell_cast,
    te_spell_damage, te_spell_steal, te_summon, te_threat
} from "../extractor/targets";
import {
    ae_aura_application,
    ae_dispel,
    ae_interrupt,
    ae_melee_damage,
    ae_spell_cast, ae_spell_cast_or_aura_application,
    ae_spell_steal, ae_threat
} from "../extractor/abilities";
import {ce_heal, ce_spell_damage} from "../extractor/causes";
import {KnechtUpdates} from "../domain_value/knecht_updates";
import {get_threat} from "../extractor/events";
import {Subject} from "rxjs";
import {auditTime} from "rxjs/operators";

export class InstanceDataLoader {
    private static readonly UPDATE_INTERVAL: number = 60000;
    private static readonly BATCH_SIZE: number = 10000;
    private static readonly INSTANCE_EXPORT_URL: string = "/API/instance/export/:instance_meta_id/:event_type/:last_event_id";

    public event_map: Map<number, Event> = new Map();
    public spell_casts: Array<Event> = [];
    public aura_applications: Array<Event> = [];
    public melee_damage: Array<Event> = [];
    public deaths: Array<Event> = [];
    public combat_states: Array<Event> = [];
    public loot: Array<Event> = [];
    public positions: Array<Event> = [];
    public powers: Array<Event> = [];
    public interrupts: Array<Event> = [];
    public spell_steals: Array<Event> = [];
    public dispels: Array<Event> = [];
    public threat_wipes: Array<Event> = [];
    public summons: Array<Event> = [];
    public spell_damage: Array<Event> = [];
    public heal: Array<Event> = [];
    public threat: Array<Event> = [];

    public sources: Map<number, [Unit, Set<number>]> = new Map();
    public targets: Map<number, [Unit, Set<number>]> = new Map();
    public abilities: Map<number, Set<number>> = new Map();

    public initialized: boolean = false;

    private newData$: Subject<void> = new Subject();

    private container_getter: Map<number, [() => Array<Event>, (Event) => Unit, (Event) => Unit, (Event) => Array<number>]> = new Map([
        [0, [() => this.spell_casts, se_identity, te_spell_cast, ae_spell_cast]],
        [1, [() => this.deaths, se_identity, undefined, undefined]],
        [2, [() => this.combat_states, se_identity, undefined, undefined]],
        [3, [() => this.loot, se_identity, undefined, undefined]],
        [4, [() => this.positions, se_identity, undefined, undefined]],
        [5, [() => this.powers, se_identity, undefined, undefined]],
        [6, [() => this.aura_applications, se_identity, te_aura_application, ae_aura_application]],
        [7, [() => this.interrupts, se_interrupt(this.event_map), se_identity, ae_interrupt(this.event_map)]],
        [8, [() => this.spell_steals, se_spell_steal(this.event_map), te_spell_steal(this.event_map), ae_spell_steal(this.event_map)]],
        [9, [() => this.dispels, se_dispel(this.event_map), te_dispel(this.event_map), ae_dispel(this.event_map)]],
        [10, [() => this.threat_wipes, se_identity, undefined, undefined]],
        [11, [() => this.summons, se_identity, te_summon, undefined]],
        [12, [() => this.melee_damage, se_identity, te_melee_damage, ae_melee_damage]],
        [13, [() => this.spell_damage, se_aura_app_or_own(ce_spell_damage, this.event_map), te_spell_damage, ae_spell_cast_or_aura_application(ce_spell_damage, this.event_map)]],
        [14, [() => this.heal, se_aura_app_or_own(ce_heal, this.event_map), te_heal, ae_spell_cast_or_aura_application(ce_heal, this.event_map)]],
        [15, [() => this.threat, se_identity, te_threat, ae_threat(this.event_map)]],
    ]);

    constructor(private instance_meta_id: number, event_types: Array<number>) {
        this.newData$.asObservable().pipe(auditTime(100))
            .subscribe(() => (self as any).postMessage(["KNECHT_UPDATES", KnechtUpdates.NewData]));
        this.load_data(event_types)
            .finally(() =>
                setInterval(() =>
                    this.load_data(event_types), InstanceDataLoader.UPDATE_INTERVAL));
    }

    private extract_subjects(event_type: number, event: Event): void {
        const getter = this.container_getter.get(event_type);
        const source = getter[1](event);
        const source_id = get_unit_id(source);
        if (this.sources.has(source_id)) this.sources.get(source_id)[1].add(event.timestamp);
        else this.sources.set(source_id, [source, new Set([event.timestamp])]);

        if (!!getter[2]) {
            const target = getter[2](event);
            if (!!target) {
                const target_id = get_unit_id(target);
                if (this.targets.has(target_id)) this.targets.get(target_id)[1].add(event.timestamp);
                else this.targets.set(target_id, [target, new Set([event.timestamp])]);
            }
        }

        if (!!getter[3]) {
            const abilities = getter[3](event);
            if (!!abilities) {
                for (const ability_id of abilities) {
                    if (ability_id >= 0) {
                        if (this.abilities.has(ability_id)) this.abilities.get(ability_id).add(event.timestamp);
                        else this.abilities.set(ability_id, new Set([event.timestamp]));
                    }
                }
            }
        }
    }

    private async load_data(event_types: Array<number>): Promise<void> {
        for (const event_type of event_types)
            await this.load_ressource(event_type);
        if (!this.initialized) {
            this.initialized = true;
            (self as any).postMessage(["KNECHT_UPDATES", KnechtUpdates.Initialized]);
        }
    }

    private async load_ressource(event_type: number): Promise<void> {
        const container = this.container_getter.get(event_type)[0]();
        const load_non_blocking = async (resolve) => {
            const container_length = container.length;
            const last_event_id = container_length === 0 ? 0 : container[container_length - 1].id;
            let result = await this.load_instance_data(event_type, last_event_id);
            const prev_result_length = result.length;
            if (event_type === 15)
                result = result.filter(event => this.event_map.has(get_threat(event).cause_event_id));
            container.push(...result);
            for (let i = container_length; i < container_length + result.length; ++i) {
                const event = container[i];
                this.extract_subjects(event_type, event);
                this.event_map.set(event.id, event);
            }
            if (result.length > 0) this.newData$.next();
            if (prev_result_length < InstanceDataLoader.BATCH_SIZE)
                resolve();
            else setTimeout(() => load_non_blocking(resolve), 100);
        };
        return new Promise<void>((resolve, reject) => load_non_blocking(resolve));
    }

    private async load_instance_data(event_type: number, last_event_id: number): Promise<Array<Event>> {
        return await (await fetch(InstanceDataLoader.INSTANCE_EXPORT_URL
            .replace(":instance_meta_id", this.instance_meta_id.toString())
            .replace(":event_type", event_type.toString())
            .replace(":last_event_id", last_event_id.toString()))).json();
    }
}
