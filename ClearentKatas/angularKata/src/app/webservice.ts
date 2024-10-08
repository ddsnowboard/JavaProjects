import { Injectable } from '@angular/core';
import { Headers, Http, RequestOptions } from '@angular/http';
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
    let headers = new Headers();
    headers.append("Content-type", "application/json");
    if(merchant.merchantId == 0)
      this.http.post(API_URL, merchant, new RequestOptions(headers)).subscribe(function(m) {});
    else
      this.http.put(API_URL + merchant.merchantId, merchant, new RequestOptions(headers)).subscribe(function(m) {});
  }

  deleteMerchant(merchant: Merchant) {
    this.http.delete(API_URL + merchant.merchantId).subscribe(function(m) {});;
  }
}
