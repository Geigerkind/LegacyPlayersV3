import {Injectable, OnDestroy} from "@angular/core";
import {TinyUrlService} from "../../../../../../tiny_url/service/tiny_url";
import {InstanceDataService} from "../../../../../service/instance_data";
import {Router} from "@angular/router";
import {InstanceViewerMeta} from "../../../../../domain_value/instance_viewer_meta";
import {Subscription} from "rxjs";
import {ViewerMode} from "../../../../../domain_value/viewer_mode";
import {RaidConfigurationService} from "../../../service/raid_configuration";
import {SettingsService} from "../../../../../../../service/settings";
import {KnechtUpdates} from "../../../../../domain_value/knecht_updates";

@Injectable({
    providedIn: "root",
})
export class ExportViewerService implements OnDestroy {

    private subscription: Subscription;

    private current_meta: InstanceViewerMeta;
    private viewer_mode: ViewerMode;

    constructor(
        private routerService: Router,
        private instanceDataService: InstanceDataService,
        private tinyUrlService: TinyUrlService,
        private raidConfigurationService: RaidConfigurationService,
        private settingsService: SettingsService
    ) {
        this.subscription = this.instanceDataService.meta.subscribe(meta => this.current_meta = meta);
        this.subscription.add(this.instanceDataService.knecht_updates.subscribe(([updates]) => {
            if (updates.includes(KnechtUpdates.FilterInitialized)) {
                this.import();
            }
        }))
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
    }

    public export(): void {
        this.tinyUrlService.set_tiny_url({
            type_id: 3,
            navigation_id: 9,
            url_suffix: this.current_meta.instance_meta_id.toString() + "/" + this.viewer_mode.toLowerCase(),
            payload: {
                instance_meta_id: this.current_meta.instance_meta_id,
                selected_categories: [...this.raidConfigurationService.category_filter.values()],
                selected_segments: [...this.raidConfigurationService.segment_filter.values()],
                selected_sources: [...this.raidConfigurationService.source_filter.values()],
                selected_targets: [...this.raidConfigurationService.target_filter.values()],
                selected_abilities: [...this.raidConfigurationService.ability_filter.values()],
            }
        });
    }

    public import(): void {
        const settings_key = "viewer_export:" + this.current_meta.instance_meta_id;
        if (!this.settingsService.check(settings_key))
            return;
        const payload = this.settingsService.get(settings_key);
        this.settingsService.delete(settings_key);
        this.raidConfigurationService.update_category_filter(payload.selected_categories);
        this.raidConfigurationService.update_segment_filter(payload.selected_segments);
        this.raidConfigurationService.update_source_filter(payload.selected_sources);
        this.raidConfigurationService.update_target_filter(payload.selected_targets);
        this.raidConfigurationService.update_ability_filter(payload.selected_abilities);
        this.raidConfigurationService.selection_overwrite$.next({
            viewer_mode: this.viewer_mode,
            categories: new Set(payload.selected_categories),
            segments: new Set(payload.selected_segments),
            sources: new Set(payload.selected_sources),
            targets: new Set(payload.selected_targets),
            abilities: new Set(payload.selected_abilities)
        });
    }

    public setViewerMode(viewer_mode: ViewerMode): void {
        this.viewer_mode = viewer_mode;
    }
}
