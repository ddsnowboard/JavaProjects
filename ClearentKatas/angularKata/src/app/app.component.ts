import { Component } from '@angular/core';

const MERCHANTS: Merchant[] = [
        {
            merchantId:1,
            name: "First Business",
            dba: "The Bar",
            mcc: "5813",
            averageTicket: 10.1000,
            annualVolume: 1000000.1000,
            address: 
            {addressId: 1,
                merchantId: 1,
                street1: "123 Main St",
                street2: "Suite 56",
                city: "Clayton",
                state: "MO",
                zip: "63105"}
        },
        {
            merchantId: 2,
            name: "Second Business",
            dba: "The Restaurant",
            mcc: "5814",
            averageTicket: 20.2000,
            annualVolume: 2000000.2000,
            address: 
            {addressId: 2,
                merchantId: 2,
                street1: "456 Elm St",
                street2: "Suite 99",
                city: "U City",
                state: "MO",
                zip: "63130"}
        }
    ];


export class Address {
    addressId: number;
    merchantId: number;
    street1: string;
    street2: string;
    city: string;
    state: string;
    zip: string;
}
export class Merchant {
    merchantId: number;
    name: string;
    dba: string;
    mcc: string;
    averageTicket: number;
    annualVolume: number;
    address: Address;
}

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
})
export class AppComponent {
    merchants: Merchant[] = MERCHANTS;
}
