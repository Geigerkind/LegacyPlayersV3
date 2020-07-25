import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {BehaviorSubject, Observable} from "rxjs";
import {Character} from "../domain_value/character";
import {
    create_array_from_behavior_subject_map,
    get_behavior_subject_map_from_array
} from "../../../stdlib/map_persistance";
import {SettingsService} from "src/app/service/settings";

@Injectable({
    providedIn: "root",
})
export class CharacterService {

    private static readonly URL_CHARACTER: string = "/armory/character/:character_id";

    private characters$: Map<number, BehaviorSubject<Character>>;

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

        const subject = new BehaviorSubject<Character>(undefined);
        this.characters$.set(character_id, subject);

        this.apiService.get(CharacterService.URL_CHARACTER
                .replace(":character_id", character_id.toString()),
            character => {
                this.settingsService.set_with_expiration("character_service_characters", create_array_from_behavior_subject_map(this.characters$), 0.5);
                subject.next(character);
            });

        return subject;
    }

}
