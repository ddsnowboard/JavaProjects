import { Injectable } from '@angular/core';
import { Headers, Http, XHRBackend } from '@angular/http';
import { Merchant } from './app.component';


const API_URL = "clearent.lan:4000/api/";

@Injectable()
export class MerchantService {
  constructor(private http: Http) {}

  getMerchants(): Observable<Merchant[]> {
    return this.http.get(API_URL).
    map(res => res.json());
  }

  sendMerchant(merchant: Merchant) {
    if(merchant.id == 0)
      this.http.post(API_URL, JSON.stringify(merchant));
    else
      this.http.put(API_URL + merchant.id, JSON.stringify(merchant));
  }

  deleteMerchant(merchant: Merchant) {
    this.http.delete(API_URL + merchant.id);
  }
}
