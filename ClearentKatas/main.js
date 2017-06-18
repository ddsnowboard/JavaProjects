"use strict";
function onReloadClicked() {
    const OBJECT_ID = "2093rlskdjafiu";
    const GET_API_ENDPOINT = "api/v1/getData?id=" + OBJECT_ID;
    var xhr = new XMLHttpRequest();
    xhr.open("GET", GET_API_ENDPOINT);
    xhr.onreadystatechanged = function() {
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
}
