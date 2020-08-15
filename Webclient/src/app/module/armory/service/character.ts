import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {BehaviorSubject, Observable} from "rxjs";
import {Character} from "../domain_value/character";
import {
    create_array_from_behavior_subject_map,
    get_behavior_subject_map_from_array
} from "../../../stdlib/map_persistance";
import {SettingsService} from "src/app/service/settings";
import {BasicCharacter} from "../domain_value/basic_character";

@Injectable({
    providedIn: "root",
})
export class CharacterService {

    private static readonly URL_CHARACTER: string = "/armory/character/:character_id";
    private static readonly URL_BASIC_CHARACTER: string = "/armory/character/basic/:character_id";

    private characters$: Map<number, BehaviorSubject<Character>>;
    private basic_characters$: Map<number, BehaviorSubject<BasicCharacter>>;

    constructor(
        private apiService: APIService,
        private settingsService: SettingsService
    ) {
    }

    get_character_by_id(character_id: number): Observable<Character> {
        if (!this.characters$)
            this.characters$ = get_behavior_subject_map_from_array(this.settingsService.get_or_set_with_expiration("character_service_characters", [], 0.5));


        if (this.characters$.has(character_id))
            return this.characters$.get(character_id).asObservable();

        const subject = new BehaviorSubject<Character>(this.get_default_character(character_id));
        this.characters$.set(character_id, subject);

        this.apiService.get(CharacterService.URL_CHARACTER
                .replace(":character_id", character_id.toString()),
            character => {
                subject.next(character);
                this.settingsService.set_with_expiration("character_service_characters", create_array_from_behavior_subject_map(this.characters$), 0.5);
            });

        return subject;
    }

    get_basic_character_by_id(character_id: number): Observable<BasicCharacter> {
        if (!this.basic_characters$)
            this.basic_characters$ = get_behavior_subject_map_from_array(this.settingsService.get_or_set_with_expiration("character_service_basic_characters", [], 0.5));


        if (this.basic_characters$.has(character_id))
            return this.basic_characters$.get(character_id).asObservable();

        const subject = new BehaviorSubject<BasicCharacter>(this.get_default_basic_character(character_id));
        this.basic_characters$.set(character_id, subject);

        this.apiService.get(CharacterService.URL_BASIC_CHARACTER
                .replace(":character_id", character_id.toString()),
            character => {
                if (!character.hero_class_id) {
                    const def_char = this.get_default_basic_character(character_id);
                    character.hero_class_id = def_char.hero_class_id;
                    character.spec_id = def_char.spec_id;
                    character.name = def_char.name;
                    character.race_id = def_char.race_id;
                }
                if (!character.spec_id)
                    character.spec_id = 0;

                subject.next(character);
                this.settingsService.set_with_expiration("character_service_basic_characters", create_array_from_behavior_subject_map(this.basic_characters$), 0.5);
            });

        return subject;
    }

    get_default_character(id: number): Character {
        return {history_moments: undefined, id, last_update: undefined, server_id: 0, server_uid: 0};
    }

    get_default_basic_character(id: number): BasicCharacter {
        return {hero_class_id: 1, id, name: "Unknown", race_id: 1, server_id: 0, spec_id: 0};
    }

}
