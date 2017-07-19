import { Injectable } from '@angular/core';
import { Headers, Http, XHRBackend } from '@angular/http';
import { Merchant } from './app.component';
import { Observable } from 'rxjs';
import 'rxjs/add/operator/map';


const API_URL = "http://cladevwrk03:4321/api/merchant/";
// const API_URL = "http://localhost:8000/api/merchant";

@Injectable()
export class MerchantService {
  constructor(private http: Http) {}

  getMerchants(): Observable<Merchant[]> {
    return this.http.get(API_URL).
      map(res => res.json());
  }

  sendMerchant(merchant: Merchant) {
    if(merchant.merchantId == 0)
      this.http.post(API_URL, JSON.stringify(merchant));
    else
      this.http.put(API_URL + merchant.merchantId, JSON.stringify(merchant));
  }

  deleteMerchant(merchant: Merchant) {
    this.http.delete(API_URL + merchant.merchantId);
  }
}
