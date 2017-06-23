"use strict";

function fillAddress(obj) {
    for(var part in obj) {
        let el = document.getElementById(part);
        if(el === null) 
            continue;
        el.getElementsByClassName("merchantValue")[0].innerHTML = obj[part];
    }
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

function onReloadClicked() {
    const ID = 1;
    const GET_API_ENDPOINT = "http://cladevwrk03:4321/api/merchant/1";
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            var jsonResponse = JSON.parse(xhr.responseText);
            for(var key in jsonResponse) {
                let el = document.getElementById(key);
                if(el === null) {
                    console.log(key + " was not found on the page");
                }
                else if(typeof(jsonResponse[key]) === typeof("") || typeof(jsonResponse[key]) === typeof(2)) {
                    el.getElementsByClassName("merchantValue")[0].innerHTML = jsonResponse[key];
                }
                else {
                    fillAddress(jsonResponse[key]);
                }
            }
        }
    };

    xhr.open("GET", GET_API_ENDPOINT);
    xhr.send();
}

function onSendClicked() {
    const NUMBER_ABOVE_WHICH_ARE_ERRORS = 400;
    const POST_API_ENDPOINT = "http://cladevwrk03:4321/api/merchant/";
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
