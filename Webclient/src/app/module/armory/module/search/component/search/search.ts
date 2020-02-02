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
            type_range: [{value: -1, label_key: "Armory.Search.hero_class"}],
            col_type: 1
        },
        {index: 1, filter_name: 'name', labelKey: "Armory.Search.name", type: 0, type_range: null, col_type: 0},
        {index: 2, filter_name: 'guild', labelKey: "Armory.Search.guild", type: 0, type_range: null, col_type: 0},
        {
            index: 3,
            filter_name: 'server',
            labelKey: "Armory.Search.server",
            type: 3,
            type_range: [{value: -1, label_key: "Armory.Search.server"}],
            col_type: 0
        },
        {index: 4, filter_name: 'last_updated', labelKey: "Armory.Search.last_update", type: 2, type_range: null, col_type: 1},
    ];
    character_body_columns: Array<Array<BodyColumn>> = [];
    clientSide: boolean = false;
    responsiveHeadColumns: Array<number> = [0, 1];
    responsiveModeWidthInPx: number = 840;
    num_characters: number = 0;

    constructor(
        private characterSearchService: CharacterSearchService,
        private dataService: DataService
    ) {
        this.dataService.get_all_servers((servers: Array<AvailableServer>) => servers.forEach(server => this.character_header_columns[3].type_range.push({
            value: server.id,
            label_key: server.name
        })));
        this.dataService.get_all_hero_classes((hero_classes: Array<Localized<HeroClass>>) => hero_classes.forEach(hero_class => this.character_header_columns[0].type_range.push({
            value: hero_class.base.id,
            label_key: hero_class.localization
        })));
        this.filterCharacterSearch(table_init_filter(this.character_header_columns));
    }

    filterCharacterSearch(filter: any): void {
        this.characterSearchService.search_characters(filter,
            (search_result) => {
                this.num_characters = search_result.num_items;
                this.character_body_columns = search_result.result.map(row => {
                    const body_columns: Array<BodyColumn> = [];

                    body_columns.push({
                        type: 3,
                        content: row.character.hero_class_id.toString(),
                        sub_type: 1,
                        sub_type_args: null
                    });
                    body_columns.push({
                        type: 0,
                        content: row.character.name,
                        sub_type: 2,
                        sub_type_args: {
                            character_id: row.character.character_id
                        }
                    });
                    body_columns.push({
                        type: 0,
                        content: !row.guild ? '' : row.guild.name,
                        sub_type: null,
                        sub_type_args: null
                    });
                    body_columns.push({
                        type: 3,
                        content: row.character.server_id.toString(),
                        sub_type: null,
                        sub_type_args: null
                    });
                    body_columns.push({
                        type: 2,
                        content: row.timestamp.toString(),
                        sub_type: null,
                        sub_type_args: null
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
