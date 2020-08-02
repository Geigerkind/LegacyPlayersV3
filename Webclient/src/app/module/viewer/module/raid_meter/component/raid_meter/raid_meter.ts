import {Component, Input, OnDestroy, OnInit} from "@angular/core";
import {DamageDoneService} from "../../service/damage_done";
import {RaidMeterRow} from "../../domain_value/raid_meter_row";
import {UtilService} from "../../service/util";
import {DamageTakenService} from "../../service/damage_taken";
import {SelectOption} from "../../../../../../template/input/select_input/domain_value/select_option";
import {Observable, of, Subscription} from "rxjs";
import {ChangedSubject, InstanceDataService} from "../../../../service/instance_data";
import {SettingsService} from "src/app/service/settings";
import {ActivatedRoute, Router} from "@angular/router";
import {ViewerMode} from "../../../../domain_value/viewer_mode";
import {InstanceViewerMeta} from "../../../../domain_value/instance_viewer_meta";
import {RaidConfigurationSelectionService} from "../../../raid_configuration_menu/service/raid_configuration_selection";
import {map} from "rxjs/operators";

@Component({
    selector: "RaidMeter",
    templateUrl: "./raid_meter.html",
    styleUrls: ["./raid_meter.scss"],
    providers: [
        UtilService,
        DamageDoneService,
        DamageTakenService
    ]
})
export class RaidMeterComponent implements OnDestroy, OnInit {

    private current_meta: InstanceViewerMeta;

    @Input() unique_id: string;

    private subscription: Subscription;
    private subscription_changed: Subscription;
    private subscription_total_duration: Subscription;
    private subscription_activated_route: Subscription;
    private subscription_meta: Subscription;

    private cookie_id: string;

    in_ability_mode: boolean = false;

    current_attempt_duration: number = 1;
    bars: Array<RaidMeterRow> = [];

    current_selection: number = 1;
    options: Array<SelectOption> = [{value: 1, label_key: 'Damage done'}, {value: 2, label_key: 'Damage taken'}];

    constructor(
        private activatedRouteService: ActivatedRoute,
        private routerService: Router,
        private settingsService: SettingsService,
        private instanceDataService: InstanceDataService,
        private raidConfigurationSelectionService: RaidConfigurationSelectionService,
        private damageDoneService: DamageDoneService,
        private damageTakenService: DamageTakenService
    ) {
        this.subscription_activated_route = this.activatedRouteService.paramMap.subscribe(params => {
            const new_mode = params.get("mode") === ViewerMode.Ability;
            if (this.in_ability_mode !== new_mode) {
                this.in_ability_mode = new_mode;
                this.selection_changed(this.current_selection);
            }
        });
        this.subscription_changed = this.instanceDataService.changed.subscribe((changed_subject) => {
            if ([ChangedSubject.Sources, ChangedSubject.Targets, ...this.get_changed_subjects_for_current_selection()].includes(changed_subject))
                this.selection_changed(this.current_selection);
        });
        this.subscription_total_duration = this.instanceDataService.attempt_total_duration.subscribe(duration => this.current_attempt_duration = duration / 1000);
        this.subscription_meta = this.instanceDataService.meta.subscribe(meta => this.current_meta = meta);
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
        this.subscription_changed?.unsubscribe();
        this.subscription_total_duration?.unsubscribe();
        this.subscription_activated_route?.unsubscribe();
        this.subscription_meta?.unsubscribe();
    }


    get_weighted_bar_fraction(amount: number): number {
        return amount / this.bars.reduce((acc, bar) => bar.amount > acc ? bar.amount : acc, 0);
    }

    get_total(): number {
        return this.bars.reduce((acc, bar) => acc + bar.amount, 0);
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
        switch (selection) {
            case 1:
                this.resubscribe(this.damageDoneService.rows.subscribe(rows => this.bars = rows));
                this.damageDoneService.reload(this.in_ability_mode);
                break;
            case 2:
                this.resubscribe(this.damageTakenService.rows.subscribe(rows => this.bars = rows));
                this.damageTakenService.reload(this.in_ability_mode);
                break;
        }
        this.current_selection = selection;

        if (!!this.unique_id)
            this.settingsService.set(this.cookie_id, selection);
    }

    private resubscribe(subscription: Subscription): void {
        this.subscription?.unsubscribe();
        this.subscription = subscription;
    }

    private get_changed_subjects_for_current_selection(): Array<ChangedSubject> {
        const result = [];
        switch (this.current_selection) {
            case 1:
            case 2:
                result.push(ChangedSubject.SpellCast);
                result.push(ChangedSubject.MeleeDamage);
                result.push(ChangedSubject.SpellDamage);
                break;
        }

        return result;
    }

    get_router_link(bar: RaidMeterRow): string {
        if (this.in_ability_mode)
            return 'detail/' + this.current_selection.toString() + '/' + bar.subject.id.toString();
        return 'ability';
    }

    bar_clicked(bar: RaidMeterRow): void {
        if (!this.in_ability_mode)
            this.raidConfigurationSelectionService.select_sources([bar.subject.id]);
        this.routerService.navigate(['/viewer/' + this.current_meta?.instance_meta_id + '/' + this.get_router_link(bar)]);
    }

    get_bar_args(bar: RaidMeterRow): Observable<Array<RaidMeterRow>> {
        this.damageDoneService.reload_preview();
        return this.damageDoneService.preview.pipe(map(preview => preview.get(bar.subject.id)));
    }

}
