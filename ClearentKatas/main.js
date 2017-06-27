"use strict";

// const GET_API_ENDPOINT = "http://cladevwrk03:4321/api/merchant/";
const GET_API_ENDPOINT = "http://localhost:8000/api/merchant/";
const POST_API_ENDPOINT = GET_API_ENDPOINT;
const NUMBER_ABOVE_WHICH_ARE_ERRORS = 400;

function fillAddress(el, obj) {
    el.querySelectorAll(".merchantDatum").forEach(function(el) {
        let value = el.getElementsByClassName("merchantValue")[0];
        // One of these will work
        value.innerHTML = value.value = obj[el.dataset.indexedBy];
    });
}

function readAddress() {
    var elements = document.getElementById("address").getElementsByClassName("merchantDatum");
    var retval = {};
    for(var idx = 0; idx < elements.length; idx++) {
        let el = elements[idx];
        retval[el.id] = el.getElementsByClassName("merchantValue")[0].value;
    }
    return retval;
}

function getRequest(url, callback) {
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            var jsonResponse = JSON.parse(xhr.responseText);
            callback(jsonResponse);
        }
    };
    xhr.open("GET", url);
    xhr.send();
}

function postRequest(url, callback, data) {
    if(typeof(data) !== typeof("")) {
        data = JSON.stringify(data);
    }

    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            var jsonResponse = JSON.parse(xhr.responseText);
            callback(jsonResponse);
        }
    };
    xhr.open("POST", url);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.send(data);
}

function putRequest(url, callback, data) {
    if(typeof(data) !== typeof("")) {
        data = JSON.stringify(data);
    }

    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            var jsonResponse = JSON.parse(xhr.responseText);
            callback(jsonResponse);
        }
    };
    xhr.open("PUT", url);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.send(data);
}

function getAllMerchants(callback) {
    getRequest(GET_API_ENDPOINT, callback);
}

function getMerchant(id, callback) {
    getRequest(GET_API_ENDPOINT + id.toString(), callback);
}

function loadMerchant(mid) {
    var callback = function(merchant) {
        document.querySelectorAll("body > .merchantDatum").forEach(function(el) {
            let value = el.getElementsByClassName("merchantValue")[0];
            if(el.dataset.indexedBy === "address")
                fillAddress(el, merchant.address);
            else 
                // One of these will work
                value.innerHTML = value.value = merchant[el.dataset.indexedBy];
        });
    }
    getMerchant(mid, callback);
}

function onLoad() {
    if(window.location.search !== "") {
        //                                                       Cut off the "?"
        loadMerchant(new URLSearchParams(document.location.search.substring(1)).get("mid"));
        return;
    }

    var callback = function(merchants) {
        const columns = document.querySelectorAll("thead td");
        let tbody = document.getElementById("tableBody");
        // Sometimes caching does strange stuff
        tbody.innerHTML = "";
        for(let i = 0; i < merchants.length; i++) {
            let merchant = merchants[i];
            // Fun fact: If Firefox thinks you've made an HTML mistake, it will add
            // stuff to the DOM to fix it.
            let htmlBuilder = ""
                htmlBuilder += `<tr onclick="goToMerchant(${merchant.merchantId});">`;
            columns.forEach(function(el) {
                htmlBuilder += `<td>${merchant[el.dataset.indexedBy]}</td>`;
            });
            htmlBuilder += "</tr>";
            tbody.innerHTML += htmlBuilder;
        }
    };
    getAllMerchants(callback);
}

function onSendClicked() {
    var inputs = document.getElementsByClassName("merchantValue");
    const values = {};
    values["address"] = readAddress();
    for(let i = 0; i < inputs.length; i++) {
        let el = inputs[i];
        if(el.id === "address")
            continue;
        let name = el.parentElement.id;
        values[name] = el.value;
    }

    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            document.getElementById("status").innerHTML = "You sent the data!";
        }
        else if(xhr.readyState === XMLHttpRequest.DONE && xhr.status >= NUMBER_ABOVE_WHICH_ARE_ERRORS) {
            document.getElementById("status").innerHTML = "Something bad happened; status code " + xhr.status;
        } 
    }
    xhr.open("POST", POST_API_ENDPOINT);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.send(JSON.stringify(values));
}

function goToMerchant(merchantId) {
    window.location = `getData.html?mid=${merchantId}`;
}

function onEditClicked() {
    var merchantId = new URLSearchParams(window.location.search.substring(1)).get("mid");
    window.location = `postData.html?mid=${merchantId}`;
}

window.onload = onLoad;
