import { Component, Input, Output } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';

const BLANK_MERCHANT: Merchant = {
  merchantId: 0,
  name: "",
  dba: "",
  mcc: "",
  averageTicket: 0,
  annualVolume: 0,
  address:
  {addressId: 0,
    merchantId: 0,
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""}
};

@Component({
  selector: "sidebar-component",
  templateUrl: "./sidebar.component.html"
})
export class SidebarComponent {
  @Input() merchants: Merchant[];
  expanded: boolean = false;
  selectedMerchant: Merchant = BLANK_MERCHANT;

  toggleDrawer() {
    this.expanded = !this.expanded;
  }

  setMerchant(merchant: Merchant) {
    if(merchant == null) {
      this.clearMerchant();
    }
    else {
      this.selectedMerchant = merchant;
      this.expanded = false;
    }
  }

  clearMerchant() {
    this.selectedMerchant = BLANK_MERCHANT;
  }
}
