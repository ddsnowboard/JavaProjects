import { Component, Input, Output } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';

@Component({
    selector: "merchant-component",
    templateUrl: "./merchant.component.html"
})
export class MerchantComponent {
    @Input() merchant: Merchant;
}
