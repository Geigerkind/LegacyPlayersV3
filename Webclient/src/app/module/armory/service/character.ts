import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {BehaviorSubject, Observable} from "rxjs";
import {Character} from "../domain_value/character";

@Injectable({
    providedIn: "root",
})
export class CharacterService {

    private static readonly URL_CHARACTER: string = "/armory/character/:character_id";

    private characters$: Map<number, BehaviorSubject<Character>> = new Map();

    constructor(
        private apiService: APIService
    ) {
    }

    get_character_by_id(character_id: number): Observable<Character> {
        if (this.characters$.has(character_id))
            return this.characters$.get(character_id).asObservable();

        const subject = new BehaviorSubject<Character>(undefined);
        this.characters$.set(character_id, subject);

        this.apiService.get(CharacterService.URL_CHARACTER
                .replace(":character_id", character_id.toString()),
            character => subject.next(character));

        return subject;
    }

}
