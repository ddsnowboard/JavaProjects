import { Component, Input, Output, EventEmitter } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';
import { NgModel } from '@angular/forms';

@Component({
  selector: "datum-component",
  templateUrl: "datum.component.html"
})
export class DatumComponent {
  @Input() value;
  @Output() valueChange = new EventEmitter();
  @Input() name: string;
  @Input() editing: boolean;

  renderValue() {
    if(this.value === 0)
      return "";
    else
      return this.value;
  }

  changed(newValue) {
  this.value = newValue;
  this.valueChange.emit(this.value);
  }
}

