import { Component, Input, Output } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';
import { NgModel } from '@angular/forms';

@Component({
  selector: "datum-component",
  templateUrl: "datum.component.html"
})
export class DatumComponent {
  @Input() value;
  @Input() name: string;
  @Input() editing: boolean;

  renderValue() {
    if(this.value === 0)
      return "";
    else
      return this.value;
  }
}

