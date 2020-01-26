import {Component} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {CharacterSearchService} from "../../service/character_search";
import {table_init_filter} from "../../../../../../template/table/utility/table_init_filter";
import {init_body_columns_from_result} from "../../../../../../template/table/utility/table_init_body";
import {DataService} from "../../../../../../service/data";
import {AvailableServer} from "../../../../../../domain_value/available_server";
import {Localized} from "../../../../../../domain_value/localized";
import {Race} from "../../../../../../domain_value/race";
import {HeroClass} from "../../../../../../domain_value/hero_class";

@Component({
    selector: "Search",
    templateUrl: "./search.html",
    styleUrls: ["./search.scss"]
})
export class SearchComponent {

    character_header_columns: Array<HeaderColumn> = [
        {index: 0, filter_name: 'name', labelKey: "Armory.Search.name", type: 0, type_range: null},
        {index: 1, filter_name: 'guild', labelKey: "Armory.Search.guild", type: 0, type_range: null},
        {
            index: 2,
            filter_name: 'gender',
            labelKey: "Armory.Search.gender",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.gender"}, {
                value: 0,
                labelKey: 'General.gender_0'
            }, {value: 1, labelKey: 'General.gender_1'}]
        },
        {
            index: 3,
            filter_name: 'server',
            labelKey: "Armory.Search.server",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.server"}]
        },
        {
            index: 4,
            filter_name: 'race',
            labelKey: "Armory.Search.race",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.race"}]
        },
        {
            index: 5,
            filter_name: 'faction',
            labelKey: "Armory.Search.faction",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.faction"}, {
                value: 0,
                labelKey: 'General.faction_0'
            }, {value: 1, labelKey: 'General.faction_1'}]
        },
        {
            index: 6,
            filter_name: 'hero_class',
            labelKey: "Armory.Search.hero_class",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.hero_class"}]
        },
        {index: 7, filter_name: 'last_updated', labelKey: "Armory.Search.last_update", type: 2, type_range: null},
    ];

    character_body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;

    constructor(
        private characterSearchService: CharacterSearchService,
        private dataService: DataService
    ) {
        this.dataService.get_all_servers((servers: AvailableServer[]) => servers.forEach(server => this.character_header_columns[3].type_range.push({
            value: server.id,
            labelKey: server.name
        })));
        this.dataService.get_all_races((races: Localized<Race>[]) => races.forEach(race => this.character_header_columns[4].type_range.push({
            value: race.base.id,
            labelKey: race.localization
        })));
        this.dataService.get_all_hero_classes((hero_classes: Localized<HeroClass>[]) => hero_classes.forEach(hero_class => this.character_header_columns[6].type_range.push({
            value: hero_class.base.id,
            labelKey: hero_class.localization
        })));
        this.filterCharacterSearch(table_init_filter(this.character_header_columns));
    }

    filterCharacterSearch(filter: any): void {
        this.characterSearchService.search_characters(filter,
            (result) => this.character_body_columns = init_body_columns_from_result(result, this.character_header_columns),
            () => {
            });
    }

}
