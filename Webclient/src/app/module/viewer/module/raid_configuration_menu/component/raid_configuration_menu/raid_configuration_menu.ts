import {Component, OnDestroy} from "@angular/core";
import {RaidConfigurationService} from "../../service/raid_configuration";
import {Subscription} from "rxjs";
import {DateService} from "../../../../../../service/date";
import {Category} from "../../domain_value/category";
import {Segment} from "../../domain_value/segment";
import {EventSource} from "../../domain_value/event_source";
import {RaidConfigurationSelectionService} from "../../service/raid_configuration_selection";
import {EventAbility} from "../../domain_value/event_ability";

@Component({
    selector: "RaidConfigurationMenu",
    templateUrl: "./raid_configuration_menu.html",
    styleUrls: ["./raid_configuration_menu.scss"]
})
export class RaidConfigurationMenuComponent implements OnDestroy {

    private subscription_categories: Subscription;
    private subscription_segments: Subscription;
    private subscription_sources: Subscription;
    private subscription_targets: Subscription;
    private subscription_abilities: Subscription;

    private subscription_source_selection: Subscription;

    closed: boolean = true;

    selected_items_categories: Array<any> = [];
    selected_items_segments: Array<any> = [];
    selected_items_sources: Array<any> = [];
    selected_items_targets: Array<any> = [];
    selected_items_abilities: Array<any> = [];

    list_items_categories: Array<any> = [];
    list_items_segments: Array<any> = [];
    list_items_sources: Array<any> = [];
    list_items_targets: Array<any> = [];
    list_items_abilities: Array<any> = [];

    use_default_filter_categories: boolean = true;
    use_default_filter_segments: boolean = true;
    use_default_filter_sources: boolean = true;
    use_default_filter_targets: boolean = true;
    use_default_filter_abilities: boolean = true;

    constructor(
        private raidConfigurationService: RaidConfigurationService,
        private raidConfigurationSelectionService: RaidConfigurationSelectionService,
        private dateService: DateService
    ) {
        this.subscription_categories = this.raidConfigurationService.categories.subscribe(categories => this.handle_categories(categories));
        this.subscription_segments = this.raidConfigurationService.segments.subscribe(segments => this.handle_segments(segments));
        this.subscription_sources = this.raidConfigurationService.sources.subscribe(sources => this.handle_sources(sources, true));
        this.subscription_targets = this.raidConfigurationService.targets.subscribe(targets => this.handle_targets(targets, true));
        this.subscription_abilities = this.raidConfigurationService.abilities.subscribe(abilities => this.handle_abilities(abilities, true));
        this.subscription_source_selection = this.raidConfigurationSelectionService.source_selection.subscribe(selection =>
            this.selected_items_sources = this.selected_items_sources.filter(item => selection.includes(item.id)));
    }

    ngOnDestroy(): void {
        this.subscription_categories?.unsubscribe();
        this.subscription_segments?.unsubscribe();
        this.subscription_sources?.unsubscribe();
        this.subscription_targets?.unsubscribe();
        this.subscription_abilities?.unsubscribe();
        this.subscription_source_selection?.unsubscribe();
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

    on_ability_selection_updated(): void {
        this.use_default_filter_targets = false;
        this.raidConfigurationService.update_ability_filter(this.selected_items_abilities.map(ability => ability.id));
    }

    private async handle_categories(categories: Array<Category>) {
        const new_list_items = [];
        for (const category of categories) {
            new_list_items.push({
                id: category.id,
                label: category.label + " - " + this.dateService.toTimeSpan(category.time)
            });
        }
        const new_selected_items = [];
        if (this.use_default_filter_categories) {
            for (const list_item of new_list_items)
                new_selected_items.push(list_item);
        } else {
            for (const selected_item of this.selected_items_categories) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_selected_items.push(selected_item);
            }
        }
        this.list_items_categories = new_list_items;
        this.selected_items_categories = new_selected_items;

        if (this.use_default_filter_categories)
            this.raidConfigurationService.update_category_filter(this.selected_items_categories.map(item => item.id));
    }

    private async handle_segments(segments: Array<Segment>) {
        const new_list_items = [];
        for (const segment of segments) {
            new_list_items.push({
                id: segment.id,
                label: segment.label + " - " + this.dateService.toTimeSpan(segment.duration) + " - "
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
                    new_selected_items.push(selected_item);
            }
        }
        this.list_items_segments = new_list_items;
        this.selected_items_segments = new_selected_items;

        if (this.use_default_filter_segments)
            this.raidConfigurationService.update_segment_filter(this.selected_items_segments.map(item => item.id));
    }

    private async handle_sources(sources: Array<EventSource>, update_filter: boolean) {
        const new_list_items = [];
        const new_selected_items = [];
        for (const source of sources) {
            source.label.subscribe(() => this.handle_sources(sources, false));
            const list_item = {
                id: source.id,
                label: source.label.toString()
            };
            new_list_items.push(list_item);
            if (this.use_default_filter_sources && source.is_player)
                new_selected_items.push(list_item);
        }
        if (!this.use_default_filter_sources) {
            for (const selected_item of this.selected_items_sources) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_selected_items.push(selected_item);
            }
        }
        this.list_items_sources = new_list_items;
        this.selected_items_sources = new_selected_items;

        if (this.use_default_filter_sources && update_filter)
            this.raidConfigurationService.update_source_filter(this.selected_items_sources.map(source => source.id));
    }

    private async handle_targets(targets: Array<EventSource>, update_filter: boolean) {
        const new_list_items = [];
        const new_selected_items = [];
        for (const target of targets) {
            target.label.subscribe(() => this.handle_targets(targets, false));
            const list_item = {
                id: target.id,
                label: target.label.toString()
            };
            new_list_items.push(list_item);

            if (this.use_default_filter_targets && !target.is_player)
                new_selected_items.push(list_item);

        }
        if (!this.use_default_filter_targets) {
            for (const selected_item of this.selected_items_targets) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_selected_items.push(selected_item);
            }
        }
        this.list_items_targets = new_list_items;
        this.selected_items_targets = new_selected_items;

        if (this.use_default_filter_targets && update_filter)
            this.raidConfigurationService.update_target_filter(this.selected_items_targets.map(target => target.id));
    }

    private async handle_abilities(abilities: Array<EventAbility>, update_filter: boolean) {
        const new_list_items = [];
        const new_selected_items = [];
        for (const ability of abilities) {
            ability.label.subscribe(() => this.handle_abilities(abilities, false));
            const list_item = {
                id: ability.id,
                label: ability.label.toString() + " (Id: " + ability.id.toString() + ")"
            };
            new_list_items.push(list_item);

            if (this.use_default_filter_abilities)
                new_selected_items.push(list_item);

        }
        if (!this.use_default_filter_abilities) {
            for (const selected_item of this.selected_items_abilities) {
                if (new_list_items.find(item => item.id === selected_item.id))
                    new_selected_items.push(selected_item);
            }
        }
        this.list_items_abilities = new_list_items;
        this.selected_items_abilities = new_selected_items;

        if (this.use_default_filter_abilities && update_filter)
            this.raidConfigurationService.update_ability_filter(this.selected_items_abilities.map(ability => ability.id));
    }
}
