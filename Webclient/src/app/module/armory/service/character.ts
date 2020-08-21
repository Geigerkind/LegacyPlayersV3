import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {Observable, of, Subject} from "rxjs";
import {Character} from "../domain_value/character";
import {BasicCharacter} from "../domain_value/basic_character";

@Injectable({
    providedIn: "root",
})
export class CharacterService {

    private static readonly URL_CHARACTER: string = "/armory/character/:character_id";
    private static readonly URL_BASIC_CHARACTER: string = "/armory/character/basic/:character_id";

    private cache_basic_character: Map<number, BasicCharacter> = new Map();

    constructor(
        private apiService: APIService
    ) {
    }

    get_character_by_id(character_id: number): Observable<Character> {
        const subject = new Subject<Character>();

        this.apiService.get(CharacterService.URL_CHARACTER
                .replace(":character_id", character_id.toString()),
            character => subject.next(character), () => subject.next(this.get_default_character(character_id)));

        return subject;
    }

    get_basic_character_by_id(character_id: number): Observable<BasicCharacter> {
        if (this.cache_basic_character.has(character_id))
            return of(this.cache_basic_character.get(character_id));

        const subject = new Subject<BasicCharacter>();

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

                this.cache_basic_character.set(character_id, character);
                subject.next(character);
            }, () => subject.next(this.get_default_basic_character(character_id)));

        return subject;
    }

    get_default_character(id: number): Character {
        return {history_moments: undefined, id, last_update: undefined, server_id: 0, server_uid: 0};
    }

    get_default_basic_character(id: number): BasicCharacter {
        return {hero_class_id: 1, id, name: "Unknown", race_id: 1, server_id: 0, spec_id: 0};
    }

}
