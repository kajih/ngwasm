import { Component, OnInit } from '@angular/core';
import * as wapp from 'wapp';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  apptext = wapp.greet();

  constructor() { }

  ngOnInit() {
    wapp.append();
    wapp.modify();
    console.log("component has been initialized!");
  }
}

