import {Component, OnDestroy} from "@angular/core";
import {RaidConfigurationService} from "../../service/raid_configuration";
import {Subscription} from "rxjs";
import {DateService} from "../../../../../../service/date";
import {Category} from "../../domain_value/category";
import {Segment} from "../../domain_value/segment";
import {EventSource} from "../../domain_value/event_source";

@Component({
    selector: "RaidConfigurationMenu",
    templateUrl: "./raid_configuration_menu.html",
    styleUrls: ["./raid_configuration_menu.scss"],
    providers: [
        RaidConfigurationService
    ]
})
export class RaidConfigurationMenuComponent implements OnDestroy {

    private subscription_categories: Subscription;
    private subscription_segments: Subscription;
    private subscription_sources: Subscription;
    private subscription_targets: Subscription;

    closed: boolean = true;

    selected_items_categories: Array<any> = [];
    selected_items_segments: Array<any> = [];
    selected_items_sources: Array<any> = [];
    selected_items_targets: Array<any> = [];

    list_items_categories: Array<any> = [];
    list_items_segments: Array<any> = [];
    list_items_sources: Array<any> = [];
    list_items_targets: Array<any> = [];

    use_default_filter_categories: boolean = true;
    use_default_filter_segments: boolean = true;
    use_default_filter_sources: boolean = true;
    use_default_filter_targets: boolean = true;

    constructor(
        private raidConfigurationService: RaidConfigurationService,
        private dateService: DateService
    ) {
        this.subscription_categories = this.raidConfigurationService.categories.subscribe(this.handle_categories);
        this.subscription_segments = this.raidConfigurationService.segments.subscribe(this.handle_segments);
        this.subscription_sources = this.raidConfigurationService.sources.subscribe(this.handle_sources);
        this.subscription_targets = this.raidConfigurationService.targets.subscribe(this.handle_targets);
    }

    ngOnDestroy(): void {
        this.subscription_categories?.unsubscribe();
        this.subscription_segments?.unsubscribe();
        this.subscription_sources?.unsubscribe();
        this.subscription_targets?.unsubscribe();
    }

    on_category_selection_updated(): void {
        this.use_default_filter_categories = false;
        this.raidConfigurationService.update_category_filter(this.selected_items_categories.map(category => category.id));
    }

    on_segment_selection_updated(): void {
        this.use_default_filter_segments = false;
        this.raidConfigurationService.update_segment_filter(this.selected_items_segments.map(segment => segment.id));
    }

    on_source_selection_updated(): void {
        this.use_default_filter_sources = false;
        this.raidConfigurationService.update_source_filter(this.selected_items_sources.map(source => source.id));
    }

    on_target_selection_updated(): void {
        this.use_default_filter_targets = false;
        this.raidConfigurationService.update_target_filter(this.selected_items_targets.map(target => target.id));
    }

    private async handle_categories(categories: Array<Category>) {
        const new_list_items = [];
        for (const category of categories) {
            new_list_items.push({
                id: category.id,
                label: (await category.label.toPromise()) + " - " + this.dateService.toTimeSpan(category.time)
            });
        }
        const new_selected_items = [];
        if (this.use_default_filter_categories) {
            for (const list_item of new_list_items)
                new_selected_items.push(list_item);
        } else {
            for (const selected_item of this.selected_items_segments) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_list_items.push(selected_item);
            }
        }
        this.list_items_categories = new_list_items;
        this.selected_items_categories = new_selected_items;
    }

    private async handle_segments(segments: Array<Segment>) {
        const new_list_items = [];
        for (const segment of segments) {
            new_list_items.push({
                id: segment.id,
                label: (await segment.label.toPromise()) + " - " + this.dateService.toTimeSpan(segment.duration) + " - "
                    + (segment.is_kill ? "Kill" : "Attempt") + " - " + this.dateService.toRPLLTime(segment.start_ts)
            });
        }
        const new_selected_items = [];
        if (this.use_default_filter_segments) {
            for (const list_item of new_list_items)
                new_selected_items.push(list_item);
        } else {
            for (const selected_item of this.selected_items_segments) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_list_items.push(selected_item);
            }
        }
        this.list_items_segments = new_list_items;
        this.selected_items_segments = new_selected_items;
    }

    private async handle_sources(sources: Array<EventSource>) {
        const new_list_items = [];
        const new_selected_items = [];
        for (const source of sources) {
            const list_item = {
                id: source.id,
                label: (await source.label.toPromise())
            };
            new_list_items.push(list_item);
            if (this.use_default_filter_sources && source.is_player)
                new_selected_items.push(list_item);
        }
        if (!this.use_default_filter_sources) {
            for (const selected_item of this.selected_items_segments) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_list_items.push(selected_item);
            }
        }
        this.list_items_sources = new_list_items;
        this.selected_items_sources = new_selected_items;

        if (this.use_default_filter_sources)
            this.raidConfigurationService.update_source_filter(this.selected_items_sources.map(source => source.id));
    }

    private async handle_targets(targets: Array<EventSource>) {
        const new_list_items = [];
        const new_selected_items = [];
        for (const target of targets) {
            const list_item = {
                id: target.id,
                label: (await target.label.toPromise())
            };
            new_list_items.push(list_item);

            if (this.use_default_filter_targets && !target.is_player)
                new_selected_items.push(list_item);

        }
        if (!this.use_default_filter_targets) {
            for (const selected_item of this.selected_items_segments) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_list_items.push(selected_item);
            }
        }
        this.list_items_targets = new_list_items;
        this.selected_items_targets = new_selected_items;

        if (this.use_default_filter_targets)
            this.raidConfigurationService.update_target_filter(this.selected_items_targets.map(target => target.id));
    }
}
