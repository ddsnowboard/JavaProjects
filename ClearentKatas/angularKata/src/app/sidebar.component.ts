import { Component, Input, Output, OnInit } from '@angular/core';
import { AppComponent, Merchant, Address } from './app.component';
import { MerchantService } from './webservice';
import { MerchantComponent } from './merchant.component';

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
  templateUrl: "./sidebar.component.html",
})
export class SidebarComponent implements OnInit {
  @Input() merchants: Merchant[];
  @Input() editor: MerchantComponent;
  expanded: boolean = false;
  selectedMerchant: Merchant = BLANK_MERCHANT;

  constructor(private merchantService: MerchantService) {}

  toggleDrawer() {
    this.expanded = !this.expanded;
  }

  ngOnInit() {

    let that = this;
    this.merchantService.getMerchants().subscribe(function(m: Merchant[]) {
      console.log(m);
      that.merchants.length = 0;
      for(let i = 0; i < m.length; i++) {
        that.merchants.push(m[i]);
      }
    });

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
    this.expanded = false;
    this.selectedMerchant = BLANK_MERCHANT;
  }

  deleteMerchant(merchant: Merchant) {
    this.clearMerchant();
    this.merchantService.deleteMerchant(merchant);
    this.ngOnInit();
  }
}
