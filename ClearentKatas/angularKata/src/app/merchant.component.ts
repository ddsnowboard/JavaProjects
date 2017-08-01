import { Component, Input, Output } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';
import { MerchantService } from "./webservice";

@Component({
  selector: "merchant-component",
  templateUrl: "./merchant.component.html",
  providers: [MerchantService]
})
export class MerchantComponent {
  @Input() merchant: Merchant;
  editing: boolean = false;

  constructor(private merchantService: MerchantService) {}

  onEditClicked() {
    this.editing = true;
  }

  onCancelClicked() {
    this.editing = false;
  }

  onSendClicked() {
    this.editing = false;
    console.log(this.merchant);
    this.merchantService.sendMerchant(this.merchant);
  }
}
