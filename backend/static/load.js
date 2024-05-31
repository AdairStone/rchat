function initializeChat() {
    var chatDiv = document.createElement('div');
    chatDiv.id = 'adabibichatapp';
    // 将div添加到body中
    document.body.appendChild(chatDiv);
    const script = document.createElement('script');
    script.src = "http://testh5.adabibi.com/chat.js";
    script.type = "module"
    document.body.appendChild(script);
    const link = document.createElement('link');
    link.rel = "modulepreload";
    link.href = "http://testh5.adabibi.com/assets/vue-CjV5JOfv.js";
    const link1 = document.createElement('link');
    link1.rel = "stylesheet";
    link1.href = "http://testh5.adabibi.com/assets/index-DaWlnu3T.css";
    document.body.appendChild(link);
    document.body.appendChild(link1);
}

document.addEventListener('DOMContentLoaded', function() {
    initializeChat();
});