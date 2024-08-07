<template>
    <div>
        <div v-for="(message, index) in messages" :key="index" :class="{ 'user-message': message.user }">
            <p>{{ message.text }}</p>
            <span>{{ message.time }}</span>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, onBeforeUnmount } from 'vue';
import { useMessageStore } from '../stores/messageStore';

export default defineComponent({
    name: 'MessagesComponent',
    setup() {
        const messageStore = useMessageStore();
        const messages = messageStore.allMessages;
        let socket: WebSocket;
        const connectWebSocket = () => {
            socket = new WebSocket('wss://your-websocket-server-url');

            socket.onmessage = (event) => {
                const data = JSON.parse(event.data);
                const newMessage = {
                    text: data.text,
                    time: new Date(data.time).toLocaleTimeString(),
                    user: data.user,
                };
                messageStore.addMessage(newMessage);
            };

            socket.onopen = () => {
                console.log('WebSocket connection established');
            };

            socket.onclose = () => {
                console.log('WebSocket connection closed');
            };

            socket.onerror = (error) => {
                console.error('WebSocket error:', error);
            };
        };

        onMounted(() => {
            connectWebSocket();
        });

        onBeforeUnmount(() => {
            if (socket) {
                socket.close();
            }
        });

        return {
            messages,
        };
    },
});
</script>

<style scoped>
.user-message {
    background-color: #d3f8d3;
    text-align: right;
}

p {
    margin: 0;
}

span {
    font-size: 0.8em;
    color: #888;
}
</style>