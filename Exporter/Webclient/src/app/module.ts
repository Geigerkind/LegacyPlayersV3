import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { Routing } from './routing';
import { App } from './component/app';

@NgModule({
  declarations: [App],
  imports: [
    BrowserModule,
    Routing
  ],
  providers: [],
  bootstrap: [App]
})
export class Module { }
