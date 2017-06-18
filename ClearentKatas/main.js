"use strict";

function onReloadClicked() {
    const GET_API_ENDPOINT = "newData.json";
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if(xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            var jsonResponse = JSON.parse(xhr.responseText);
            for(var key in jsonResponse) {
                let el = document.getElementById(key);
                if(el === null) {
                    console.log(key + " was not found on the page");
                    continue;
                }
                el.getElementsByClassName("merchantValue")[0].innerHTML = jsonResponse[key];
            }
        }
    };
    xhr.open("GET", GET_API_ENDPOINT);
    xhr.send();
}

function onSendClicked() {
    const NUMBER_ABOVE_WHICH_ARE_ERRORS = 400;
    const POST_API_ENDPOINT = "iCantPostData.php";
    var inputs = document.getElementsByClassName("merchantValue");
    const values = {};
    for(let i = 0; i < inputs.length; i++) {
        let el = inputs[i];
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
