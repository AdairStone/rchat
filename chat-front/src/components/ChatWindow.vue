<template>
    <div>
        <div v-for="(message, index) in messages" :key="index">{{ message }}</div>
        <input v-model="messageInput" @keyup.enter="sendMessage" placeholder="Type a message...">
    </div>
</template>

<script type="ts">
export default {
    name: 'ChatWindow',
    data() {
        return {
            ws: null,
            messageInput: '',
            messages: [],
            server: {
                protocol: 'ws',
                hostname: '',
                port: '5173'
            }
        };
    },
    mounted() {
        const protocol = window.location.protocol;
        if (protocol == 'https:') {
            this.server.protocol = 'wss';
        }
        const hostname = window.location.hostname;
        this.server.hostname = hostname
        const port = window.location.port;
        this.server.port = port
        this.connectWebSocket(this.server);

    },
    methods: {
        connectWebSocket(server) {
            // const wsServer = server.protocol + '://' + server.hostname + ':' + server.port + '/socket'
            const wsServer = 'ws://127.0.0.1:6080/clientchat';
            console.log('connect to ws server:', wsServer)
            this.ws = new WebSocket(wsServer);
            this.ws.addEventListener('open', () => {
                console.log('WebSocket connection established');
            });
            this.ws.addEventListener('message', (event) => {
                this.messages.push(event.data);
            });
        },
        sendMessage() {
            if (this.messageInput.trim() !== '') {
                this.ws.send(this.messageInput);
                this.messages.push('You: ' + this.messageInput);
                this.messageInput = '';
            }
        }
    }
};
</script>