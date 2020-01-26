import {Component} from "@angular/core";
import {HeaderColumn} from "../../../../../../template/table/module/table_header/domain_value/header_column";
import {BodyColumn} from "../../../../../../template/table/module/table_body/domain_value/body_column";
import {CharacterSearchService} from "../../service/character_search";
import {table_init_filter} from "../../../../../../template/table/utility/table_init_filter";
import {DataService} from "../../../../../../service/data";
import {AvailableServer} from "../../../../../../domain_value/available_server";
import {Localized} from "../../../../../../domain_value/localized";
import {HeroClass} from "../../../../../../domain_value/hero_class";

@Component({
    selector: "Search",
    templateUrl: "./search.html",
    styleUrls: ["./search.scss"]
})
export class SearchComponent {

    character_header_columns: Array<HeaderColumn> = [
        {
            index: 0,
            filter_name: 'hero_class',
            labelKey: "Armory.Search.hero_class",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.hero_class"}]
        },
        {index: 1, filter_name: 'name', labelKey: "Armory.Search.name", type: 0, type_range: null},
        {index: 2, filter_name: 'guild', labelKey: "Armory.Search.guild", type: 0, type_range: null},
        {
            index: 3,
            filter_name: 'server',
            labelKey: "Armory.Search.server",
            type: 3,
            type_range: [{value: -1, labelKey: "Armory.Search.server"}]
        },
        {index: 4, filter_name: 'last_updated', labelKey: "Armory.Search.last_update", type: 2, type_range: null},
    ];
    character_body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1];
    responsiveModeWidthInPx: number = 720;

    constructor(
        private characterSearchService: CharacterSearchService,
        private dataService: DataService
    ) {
        this.dataService.get_all_servers((servers: Array<AvailableServer>) => servers.forEach(server => this.character_header_columns[3].type_range.push({
            value: server.id,
            labelKey: server.name
        })));
        this.dataService.get_all_hero_classes((hero_classes: Array<Localized<HeroClass>>) => hero_classes.forEach(hero_class => this.character_header_columns[0].type_range.push({
            value: hero_class.base.id,
            labelKey: hero_class.localization
        })));
        this.filterCharacterSearch(table_init_filter(this.character_header_columns));
    }

    filterCharacterSearch(filter: any): void {
        this.characterSearchService.search_characters(filter,
            (result) => {
                this.character_body_columns = result.map(row => {
                    const body_columns: Array<BodyColumn> = [];

                    body_columns.push({
                        type: 3,
                        content: row.character.last_update.character_info.hero_class_id.toString(),
                        sub_type: 1
                    });
                    body_columns.push({
                        type: 0,
                        content: row.character.last_update.character_name,
                        sub_type: null
                    });
                    body_columns.push({
                        type: 0,
                        content: !row.guild ? '' : row.guild.name,
                        sub_type: null
                    });
                    body_columns.push({
                        type: 3,
                        content: row.character.server_id.toString(),
                        sub_type: null
                    });
                    body_columns.push({
                        type: 2,
                        content: row.character.last_update.timestamp.toString(),
                        sub_type: null
                    });

                    // TODO: Dont save faction color here
                    return {
                        color: row.faction ? '#372727' : '#272f37',
                        columns: body_columns
                    };
                });
            },
            () => {
            });
    }

}
