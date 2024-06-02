<template>
    <div v-if="isOpen" class="modal box" @keydown="handleKeydown">
        <div class="modal-content">
            <span v-if="closeable" class="close" @click="closeModal">&times;</span>
            <div class="chat-header">
                <h2>adabibi.com客服服务</h2>
                <p>{{ currentTime }}</p>
                <a href="#">adabibi.com智能客服为您提供服务</a>
            </div>
            <div class="chat-messages" ref="chatMessage">
                <div v-for="(message, index) in messages" :key="index" :class="{ 'user-message': message.user }">
                    <p>{{ message.text }}</p>
                    <span>{{ message.time }}</span>
                </div>
            </div>
            <div class="chat-input">
                <input ref="messageInputRef" v-model="newMessage" @keydown.enter="sendMessage" placeholder="输入消息..." />
                <button @click="sendMessage">发送</button>
                <!-- <button @click="toggleEmojiPicker">😀</button> -->
            </div>
            <!-- <emoji-picker v-if="isEmojiPickerVisible" @emoji-click="addEmoji"></emoji-picker> -->
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, nextTick, watch, onMounted } from 'vue';
// import '@joeattardi/emoji-picker-element';
// 
export default defineComponent({
    name: 'ChatModal',
    props: {
        isOpen: {
            type: Boolean,
            required: true,
        },
        closeable: {
            type: Boolean,
            required: false,
            default: true,
        },
    },
    emits: ['close'],
    setup(props, { emit }) {
        const currentTime = new Date().toLocaleString();
        const newMessage = ref('');
        const messages = ref([
            { text: '你好，请问有什么问题需要帮您解决？', time: '10:33:25', user: false },
            { text: '你好，请问怎么充值', time: '10:33:55', user: true },
            { text: '在系统后台选择充值服务，可以进行充值😊', time: '10:34:12', user: false },
        ]);
        const isEmojiPickerVisible = ref(false);

        const messageInputRef = ref<HTMLInputElement | null>(null);
        const focusInput = () => {
            if (messageInputRef.value) {
                messageInputRef.value.focus();
            }
        };

        onMounted(() => {
            connect();
            nextTick(() => {
                console.log("real exe");
                focusInput();
            });
        });

        // 引用WebSocket对象
        const ws = ref<WebSocket | null>(null);

        // 连接WebSocket服务器
        const connect = () => {
            var protocol = window.location.protocol;
            if (protocol == 'https:') {
                protocol = 'wss';
            } else {
                protocol = 'ws'
            }
            const hostname = window.location.hostname;
            const port = window.location.port;

            // const url = 'wss://your-websocket-url';
            const url = 'ws://127.0.0.1:6080/clientchat?site_key=DBlDpVRp2ejdBxl&client=1';
            // 替换为你的WebSocket服务器URL
            ws.value = new WebSocket(url);

            // 监听WebSocket打开事件
            ws.value.onopen = () => {
                console.log('WebSocket 连接成功');
            };

            // 监听WebSocket消息事件
            ws.value.onmessage = (event) => {
                console.log('收到消息:', event.data);
                messages.value.push({ text: event.data, time: new Date().toLocaleString(), user: false });
                scrollToBottom();
            };

            // 监听WebSocket错误事件
            ws.value.onerror = (error) => {
                console.error('WebSocket 连接出错:', error);
            };

            // 监听WebSocket关闭事件
            ws.value.onclose = () => {
                console.log('WebSocket 连接已关闭');
            };
        };

        const sendMessage = () => {
            if (newMessage.value.trim()) {
                if (ws.value) {
                    ws.value.send(newMessage.value);
                }
                messages.value.push({
                    text: newMessage.value,
                    time: new Date().toLocaleTimeString(),
                    user: true,
                });

                newMessage.value = '';
                scrollToBottom();
                focusInput();
            }
        };

        const handleKeydown = (event: KeyboardEvent) => {
            console.log('key down')
            if (event.key === 'Escape') {
                closeModal();
            }
        }

        const closeModal = () => {
            emit('close');
        };

        const toggleEmojiPicker = () => {
            isEmojiPickerVisible.value = !isEmojiPickerVisible.value;
        };

        const addEmoji = (event: any) => {
            newMessage.value += event.detail.unicode;
        };

        const chatMessage = ref<HTMLElement | null>(null);
        // 滚动到底部的方法
        const scrollToBottom = () => {
            if (!chatMessage.value) {
                return;
            }
            nextTick(() => {
                chatMessage.value!.scrollTop = chatMessage.value!.scrollHeight;
            });
        };

        watch(() => props.isOpen, (newValue, _) => {
            if (newValue) {
                scrollToBottom();
                focusInput();
            }
        });

        return {
            currentTime,
            newMessage,
            messages,
            sendMessage,
            closeModal,
            toggleEmojiPicker,
            addEmoji,
            chatMessage,
            scrollToBottom,
            messageInputRef,
            focusInput,
            handleKeydown
        };
    },
});
</script>

<style scoped>
.modal {
    position: fixed;
    z-index: 1;
    /* right: 0; */
    bottom: 20px;
    /* height: 600px;
    width: 400px; */
    right: 20px;
    /* background-color: rgba(0, 0, 0, 0.4); */
    display: flex;
    justify-content: center;
    align-items: flex-end;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
    border-radius: 5px;
}

/* Styles for screens wider than 600px */
@media (min-width: 400px) {
    .box {
        height: 600px;
        width: 90%;
    }
}

/* Styles for screens wider than 800px */
@media (min-width: 450px) {
    .box {
        height: 600px;
        width: 400px;
    }
}

.modal-content {
    /* background-color: #fefefe; */
    padding: 10px;
    border: none;
    border-radius: 5px;
    /* border: 0px solid #888; */
    /* height: 100%; */
    /* max-width: 500px; */
    /* margin: 0 auto;   */
    width: 100%;
    transition: transform 0.3s ease;
    transform: translateY(100%);
    background-color: rgba(0, 123, 255, 0.8);
}

.modal.open {
    opacity: 1;
    pointer-events: auto;
}

.modal.open .modal-content {
    transform: translateY(0);
}

.close {
    color: #aaa;
    float: right;
    font-size: 28px;
    font-weight: bold;
}

.close:hover,
.close:focus {
    color: rgb(118, 133, 117);
    text-decoration: none;
    cursor: pointer;
}

.chat-header {
    border: none;
    text-align: left;
    padding: 10px;
    color: #f5f5f5;
}

.chat-messages {
    flex-grow: 1;
    padding: 10px;
    height: 400px;
    max-height: 400px;
    overflow-y: scroll;
    background-color: #fefefe;
    border-radius: 10px;
    border: none;
    text-align: left;
}

.chat-messages div {
    margin-bottom: 10px;
}

.user-message {
    text-align: right;
}

.chat-input {
    padding-top: 10px;
    border-top: 1px solid #ccc;
    display: flex;
    align-items: center;
    height: 50px;
    border: none;
}

.chat-input input {
    flex-grow: 1;
    padding: 5px;
    height: 40px;
    border: none;
    border-radius: 4px;
}

.chat-input input:focus {
    border-color: #00F2FF70;
    box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
    outline: none;
}

.chat-input button {
    margin-left: 10px;
    padding: 5px 10px;
    height: 40px;
    width: 60px;
    border: none;
    background-color: #007BFF;
    color: white;
    border-radius: 4px;
    cursor: pointer;
}
</style>