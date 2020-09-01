import {Event} from "../domain_value/event";
import {get_unit_id, Unit} from "../domain_value/unit";
import {
    se_aura_application, se_combat_state, se_death,
    se_heal,
    se_interrupt, se_loot, se_melee_damage, se_position, se_power, se_spell_cast,
    se_spell_damage,
    se_summon, se_threat, se_threat_wipe, se_un_aura
} from "../extractor/sources";
import {
    te_aura_application, te_heal, te_interrupt, te_melee_damage,
    te_spell_cast,
    te_spell_damage, te_summon, te_threat, te_un_aura
} from "../extractor/targets";
import {
    ae_aura_application, ae_heal,
    ae_interrupt,
    ae_melee_damage,
    ae_spell_cast, ae_spell_damage, ae_threat, ae_un_aura
} from "../extractor/abilities";
import {KnechtUpdates} from "../domain_value/knecht_updates";
import {Subject} from "rxjs";
import {auditTime} from "rxjs/operators";

export class InstanceDataLoader {
    private static readonly UPDATE_INTERVAL: number = 60000;
    private static readonly BATCH_SIZE: number = 100000;
    private static readonly INSTANCE_EXPORT_URL: string = "/API/instance/export/:instance_meta_id/:event_type/:last_event_id";

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

    private container_getter: Map<number, [() => Array<Event>, (Event) => Unit, (Event) => Unit, (Event) => number | Array<number>]> = new Map([
        [0, [() => this.spell_casts, se_spell_cast, te_spell_cast, ae_spell_cast]],
        [1, [() => this.deaths, se_death, undefined, undefined]],
        [2, [() => this.combat_states, se_combat_state, undefined, undefined]],
        [3, [() => this.loot, se_loot, undefined, undefined]],
        [4, [() => this.positions, se_position, undefined, undefined]],
        [5, [() => this.powers, se_power, undefined, undefined]],
        [6, [() => this.aura_applications, se_aura_application, te_aura_application, ae_aura_application]],
        [7, [() => this.interrupts, se_interrupt, te_interrupt, ae_interrupt]],
        [8, [() => this.spell_steals, se_un_aura, te_un_aura, ae_un_aura]],
        [9, [() => this.dispels, se_un_aura, te_un_aura, ae_un_aura]],
        [10, [() => this.threat_wipes, se_threat_wipe, undefined, undefined]],
        [11, [() => this.summons, se_summon, te_summon, undefined]],
        [12, [() => this.melee_damage, se_melee_damage, te_melee_damage, ae_melee_damage]],
        [13, [() => this.spell_damage, se_spell_damage, te_spell_damage, ae_spell_damage]],
        [14, [() => this.heal, se_heal, te_heal, ae_heal]],
        [15, [() => this.threat, se_threat, te_threat, ae_threat]],
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

        if (this.sources.has(source_id)) this.sources.get(source_id)[1].add(event[1]);
        else this.sources.set(source_id, [source, new Set([event[1]])]);

        if (!!getter[2]) {
            const target = getter[2](event);
            if (!!target) {
                const target_id = get_unit_id(target);
                if (this.targets.has(target_id)) this.targets.get(target_id)[1].add(event[1]);
                else this.targets.set(target_id, [target, new Set([event[1]])]);
            }
        }

        if (!!getter[3]) {
            const abilities = getter[3](event);
            if (!!abilities) {

                if (abilities instanceof Array) {
                    for (const ability_id of abilities) {
                        if (ability_id >= 0) {
                            if (this.abilities.has(ability_id)) this.abilities.get(ability_id).add(event[1]);
                            else this.abilities.set(ability_id, new Set([event[1]]));
                        }
                    }
                } else {
                    if (abilities >= 0) {
                        if (this.abilities.has(abilities)) this.abilities.get(abilities).add(event[1]);
                        else this.abilities.set(abilities, new Set([event[1]]));
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
            const last_event_id = container_length === 0 ? 0 : container[container_length - 1][0];
            const result = await this.load_instance_data(event_type, last_event_id);
            container.push(...result);

            for (let i = container_length; i < container_length + result.length; ++i) {
                const event = container[i];
                this.extract_subjects(event_type, event);
            }
            if (result.length > 0) this.newData$.next();
            if (result.length < InstanceDataLoader.BATCH_SIZE)
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
