"use strict";

const GET_API_ENDPOINT = "http://cladevwrk03:4321/api/merchant/";
// const GET_API_ENDPOINT = "http://localhost:8000/api/merchant/";
const POST_API_ENDPOINT = GET_API_ENDPOINT;
const NUMBER_ABOVE_WHICH_ARE_ERRORS = 400;

function getCurrentMerchantId() {
    //                                                       Cut off the "?"
    return new URLSearchParams(document.location.search.substring(1)).get("mid")
}

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
    if(mid === null)
        return;
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
    var currentMerchantId = getCurrentMerchantId();
    if(currentMerchantId != null) {
        loadMerchant(currentMerchantId);
    }
    else {
        // Clear out fields
        // Sometimes they have stuff in them and they shouldn't.
        document.querySelectorAll("input").forEach(function(el) {
            el.value = "";
        });

        document.querySelectorAll(".merchantAttribute").forEach(function(el) {
            if(el.parentNode.id != "address")
                el.innerHTML = "";
        });
    }

    var callback = function(merchants) {
        let sidebar = document.getElementById("sidebar");
        // Sometimes caching does strange stuff
        sidebar.innerHTML = "";
        for(let i = 0; i < merchants.length; i++) {
            let merchant = merchants[i];
            sidebar.innerHTML += `<div class="companyName" onclick="goToMerchant(${merchant.merchantId});">${merchant.dba}<span class="deleteButton" onclick="deleteMerchant(${merchant.merchantId})">[delete]</span></div>`;
        }
        sidebar.innerHTML += `<button onclick="createMerchant();">New Merchant</button>`;
    };
    getAllMerchants(callback);
}

function onSendClicked() {
    var inputs = document.getElementsByClassName("merchantValue");
    const values = {};
    values["address"] = readAddress();
    for(let i = 0; i < inputs.length; i++) {
        let el = inputs[i];
        if(el.parentNode.id === "address" || el.parentNode.parentNode.id === "address")
            continue;
        let name = el.parentElement.id;
        values[name] = el.value;
    }
    setViewMode();

    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        console.log(`Ready state is ${xhr.readyState} and status is ${xhr.status}`);
    }

    let httpVerb;
    let endpoint;
    if(getCurrentMerchantId() === null) {
        httpVerb = "POST"
            endpoint = POST_API_ENDPOINT;
    }
    else {
        httpVerb = "PUT";
        endpoint = POST_API_ENDPOINT + getCurrentMerchantId().toString();
        values["merchantId"] = getCurrentMerchantId();
    }
    xhr.open(httpVerb, endpoint);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.send(JSON.stringify(values));
}

function goToMerchant(merchantId) {
    history.pushState(null, "Merchant " + merchantId, `getData.html?mid=${merchantId}`);
    setViewMode();
    loadMerchant(merchantId);
    toggleDrawer();
}

function setEditingMode() {
    document.querySelectorAll(".merchantValue").forEach(function(el) {
        if(el.parentNode.dataset.indexedBy === "address")
            return;
        let newElement = document.createElement("input");
        newElement.classList.add("merchantValue");
        el.replaceWith(newElement);
    });
    loadMerchant(getCurrentMerchantId());

    var button = document.querySelector("#submit");
    button.innerHTML = "Send";
    button.onclick = onSendClicked;
}

function setViewMode() {
    document.querySelectorAll(".merchantValue").forEach(function(el) {
        if(el.parentNode.dataset.indexedBy === "address")
            return;
        let newElement = document.createElement("span");
        newElement.classList.add("merchantValue");
        el.replaceWith(newElement);
    });
    loadMerchant(getCurrentMerchantId());

    var button = document.querySelector("#submit");
    button.innerHTML = "Edit";
    button.onclick = onEditClicked;
}

function onEditClicked() {
    setEditingMode();
}

function toggleDrawer() {
    const VERTICAL = "vertical";
    const EXPANDED = "expanded";
    var hamburger =  document.getElementById("hamburger");
    var sidebar = document.getElementById("sidebar");
    if(hamburger.classList.contains(VERTICAL)) {
        // close drawer
        hamburger.classList.remove(VERTICAL);
        sidebar.classList.remove(EXPANDED);
    }
    else {
        // open drawer
        hamburger.classList.add(VERTICAL);
        sidebar.classList.add(EXPANDED);
    }
}

function createMerchant() {
    history.pushState(null, "New Merchant", "getData.html");
    setEditingMode();
    toggleDrawer();
}

function deleteMerchant(merchantId) {
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            history.replaceState(null, "Back to start", "getData.html");
            onLoad();
        }
    };
    xhr.open("DELETE", POST_API_ENDPOINT + merchantId.toString());
    xhr.send();
}

window.onload = onLoad;
window.onpopstate = onLoad;
