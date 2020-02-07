import { Component } from '@angular/core';
import {TranslationService} from "../service/translation";

@Component({
  selector: 'root',
  templateUrl: './app.html',
  styleUrls: ['./app.scss']
})
export class App {
  title = 'LP Consent Manager';

  constructor(
      private translationService: TranslationService
  ) {}
}
