function setCookie(name, value, days) {
    let expires = "";
    if (days) {
        let date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "")  + expires + "; path=/";
}

function getCookie(cookieName) {
    var name = cookieName + "=";
    var decodedCookie = decodeURIComponent(document.cookie);
    var cookieArray = decodedCookie.split(';');
    for(var i = 0; i < cookieArray.length; i++) {
        var cookie = cookieArray[i];
        while (cookie.charAt(0) == ' ') {
            cookie = cookie.substring(1);
        }
        if (cookie.indexOf(name) == 0) {
            return cookie.substring(name.length, cookie.length);
        }
    }
    return "";
}

function getQueryStringParams() {
    const params = {};
    const queryString = window.location.search.substring(1);
    const queries = queryString.split('&');

    for (const query of queries) {
        const [key, value] = query.split('=');
        params[decodeURIComponent(key)] = decodeURIComponent(value || '');
    }

    return params;
}
  
const params = getQueryStringParams();
console.log("key:",params['key']);
  
function initializeChat() {
    var chatDiv = document.createElement('div');
    chatDiv.id = 'adabibichatapp';
    // 将div添加到body中
    document.body.appendChild(chatDiv);
    const script = document.createElement('script');
    script.src = "{{script_home}}/chat.js";
    script.type = "module"
    document.body.appendChild(script);
    const link = document.createElement('link');
    link.rel = "modulepreload";
    link.href = "{{script_home}}/vue.js";
    const link1 = document.createElement('link');
    link1.rel = "stylesheet";
    link1.href = "{{script_home}}/index.css";
    document.body.appendChild(link);
    document.body.appendChild(link1);
}

document.addEventListener('DOMContentLoaded', function() {
    setCookie("bibirchat_site_key","{{site_key}}", 7);
    setCookie("bibirchat_sserver","{{script_home}}", 7);
    var ukey = getCookie("bibirchat_ukey");
    if (ukey==="") {
        setCookie("bibirchat_ukey","{{ukey}}", 7);
    }
    initializeChat();
});