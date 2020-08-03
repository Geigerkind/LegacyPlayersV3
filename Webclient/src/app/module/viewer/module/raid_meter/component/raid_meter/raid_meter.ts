import {Component, Input, OnDestroy, OnInit} from "@angular/core";
import {DamageDoneService} from "../../service/damage_done";
import {UtilService} from "../../service/util";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {Observable, of, Subscription} from "rxjs";
import {ChangedSubject, InstanceDataService} from "../../../../service/instance_data";
import {SettingsService} from "src/app/service/settings";
import {ActivatedRoute, Router} from "@angular/router";
import {ViewerMode} from "../../../../domain_value/viewer_mode";
import {InstanceViewerMeta} from "../../../../domain_value/instance_viewer_meta";
import {RaidConfigurationSelectionService} from "../../../raid_configuration_menu/service/raid_configuration_selection";
import {RaidMeterSubject} from "../../domain_value/raid_meter_subject";
import {RaidMeterService} from "../../service/raid_meter";
import {RaidDetailService} from "../../../raid_detail_table/service/raid_detail";

@Component({
    selector: "RaidMeter",
    templateUrl: "./raid_meter.html",
    styleUrls: ["./raid_meter.scss"],
    providers: [
        UtilService,
        DamageDoneService,
        RaidMeterService
    ]
})
export class RaidMeterComponent implements OnDestroy, OnInit {

    private current_meta: InstanceViewerMeta;

    @Input() unique_id: string;

    private subscription: Subscription;
    private subscription_total_duration: Subscription;
    private subscription_activated_route: Subscription;
    private subscription_meta: Subscription;
    private subscription_abilities: Subscription;
    private subscription_units: Subscription;

    private cookie_id: string;
    private current_data: Array<[number, Array<[number, number]>]> = [];
    private abilities: Map<number, RaidMeterSubject> = new Map();
    private units: Map<number, RaidMeterSubject> = new Map();

    in_ability_mode: boolean = false;

    current_attempt_duration: number = 1;
    bars: Array<[number, number]> = [];

    current_selection: number = 1;
    options: Array<SelectOption> = [{value: 1, label_key: 'Damage done'}, {value: 2, label_key: 'Damage taken'}];

    constructor(
        private activatedRouteService: ActivatedRoute,
        private routerService: Router,
        private settingsService: SettingsService,
        private instanceDataService: InstanceDataService,
        private raidConfigurationSelectionService: RaidConfigurationSelectionService,
        private raidMeterService: RaidMeterService,
        private raidDetailService: RaidDetailService
    ) {
        this.subscription_activated_route = this.activatedRouteService.paramMap.subscribe(params => {
            const new_mode = params.get("mode") === ViewerMode.Ability;
            if (this.in_ability_mode !== new_mode) {
                this.in_ability_mode = new_mode;
                this.selection_changed(this.current_selection);
            }
        });
        this.subscription_total_duration = this.instanceDataService.attempt_total_duration.subscribe(duration => this.current_attempt_duration = duration / 1000);
        this.subscription_meta = this.instanceDataService.meta.subscribe(meta => this.current_meta = meta);
        this.subscription_abilities = this.raidMeterService.abilities.subscribe(abilities => this.abilities = abilities);
        this.subscription_units = this.raidMeterService.units.subscribe(units => this.units = units);
        this.subscription = this.raidMeterService.data.subscribe(rows => {
            this.current_data = rows;
            let result;
            if (this.in_ability_mode) result = this.ability_rows(rows);
            else result = rows.map(([unit_id, abilities]) =>
                [unit_id, abilities.reduce((acc, [ability_id, amount]) => acc + amount, 0)]);
            this.bars = result.sort((left, right) => right[1] - left[1]);
        });
    }

    ngOnInit(): void {
        this.cookie_id = "raid_meter_" + this.unique_id;

        if (this.settingsService.check(this.cookie_id)) {
            this.current_selection = this.settingsService.get(this.cookie_id);
        }
        this.selection_changed(this.current_selection);
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
        this.subscription_total_duration?.unsubscribe();
        this.subscription_activated_route?.unsubscribe();
        this.subscription_meta?.unsubscribe();
        this.subscription_abilities?.unsubscribe();
        this.subscription_units?.unsubscribe();
    }


    get_weighted_bar_fraction(amount: number): number {
        return amount / this.bars.reduce((acc, bar) => bar[1] > acc ? bar[1] : acc, 0);
    }

    get_total(): number {
        return this.bars.reduce((acc, bar) => acc + bar[1], 0);
    }

    get_fraction(amount: number): number {
        return amount / this.get_total();
    }

    get_total_dps(): number {
        return this.get_total() / this.current_attempt_duration;
    }

    get_dps(amount: number): number {
        return amount / this.current_attempt_duration;
    }

    selection_changed(selection: number): void {
        this.raidMeterService.select(selection);

        if (!!this.unique_id)
            this.settingsService.set(this.cookie_id, selection);
    }

    private ability_rows(data: Array<[number, Array<[number, number]>]>): Array<[number, number]> {
        return data.reduce((acc, [unit_id, abilities]) => {
            const ability_amount = new Map();
            for (const [ability_id, amount] of abilities) {
                if (ability_amount.has(ability_id)) ability_amount.set(ability_id, ability_amount.get(ability_id) + amount);
                else ability_amount.set(ability_id, amount);
            }
            return [...ability_amount.entries()];
        }, []);
    }

    get_router_link(bar: [number, number]): string {
        if (this.in_ability_mode)
            return 'detail/' + this.current_selection.toString() + '/' + bar[0].toString();
        return 'ability';
    }

    bar_clicked(bar: [number, number]): void {
        if (!this.in_ability_mode)
            this.raidConfigurationSelectionService.select_sources([bar[0]]);
        this.routerService.navigate(['/viewer/' + this.current_meta?.instance_meta_id + '/' + this.get_router_link(bar)]);
    }

    get_bar_tooltip_payload(bar: [number, number]): Array<[Observable<string>, number]> {
        if (!this.in_ability_mode)
            return this.ability_rows(this.current_data.filter(([unit_id, abilities]) => unit_id === bar[0]))
                .sort((left, right) => right[1] - left[1])
                .map(([ability_id, amount]) => [this.abilities.get(ability_id).name, amount]);
        return []; // TODO
    }

    get_bar_subject(subject_id: number): RaidMeterSubject {
        if (this.in_ability_mode)
            return this.abilities.get(subject_id);
        return this.units.get(subject_id);
    }

}
