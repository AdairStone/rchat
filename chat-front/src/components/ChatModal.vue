<template>
    <div v-if="isOpen" class="modal box" @keydown="handleKeydown">
        <div class="modal-content">
            <span v-if="closeable" class="close" @click="closeModal">&times;</span>
            <div class="chat-header">
                <div class="chat-title">adabibi.com客服服务</div>
                <div class="chat-time">{{ currentTime }}</div>
                <div class="chat-notify">adabibi.com智能客服为您提供服务</div>
            </div>
            <div class="chat-messages" ref="chatMessage">
                <div v-for="(message, index) in messages" :key="index">
                    <div class="message-notify" v-if="message.notify && '' !== message.notify">
                        {{ message.time }}
                        <div v-html="message.notify"></div>
                    </div>
                    <div class="message-content" v-if="!message.notify || '' === message.notify">
                        <div class="message-header" :style="{ 'order': message.user ? '0' : '1' }">
                            <Icon icon="ep:avatar" width="35" height="30" style="color: #FFFFFF;" />
                        </div>
                        <div class="message-main" :class="{ 'user-message': !message.user }">
                            <div class="message-text" :style="{ 'float': message.user ? 'left' : 'right' }"
                                v-if="message.text && message.text !== ''">
                                <div style="display: inline-block;">
                                    <ul class="message-files" v-if="message.files && message.files.length > 0">
                                        <li :style="{ 'float': message.user ? 'left' : 'right' }"
                                            v-for=" i in message.files" :key="i">
                                            <img v-if="isImage(i.url)" :src="i.url" @click="openImagePreview(i.url)" />
                                            <div v-if="isVideo(i.url)">
                                                <video width="100" height="100" controls>
                                                    <source :src="i.url" type="video/mp4">hsla(160, 100%, 37%, 1
                                                    Your browser does not support the video tag.
                                                </video>
                                            </div>
                                            <div v-if="!isVideo(i.url) && !isImage(i.url)"
                                                @click="toDownload(i.url, i.name)">
                                                <Icon icon="mage:file-3" width="75" height="75" style="color: #757070">
                                                </Icon>
                                                <div style="text-align: center;">{{ i.name }}</div>
                                            </div>
                                        </li>
                                    </ul>
                                </div>
                                <div class="content">{{ message.text }}</div>
                                <span class="message-time">{{ message.time }}</span>
                            </div>
                        </div>
                    </div>
                </div>
                <ImagePreview :visible="previewVisible" :src="previewImgSrc" @close="previewVisible = false" />
            </div>
            <div class="emoji-picker-container">
                <EmojiPicker v-if="pickerShow" :data="emojiIndex" @select="addEmoji" emoji="grinning"
                    :showPreview="false" :native="true" :skin="0" title="表情包">
                </EmojiPicker>
            </div>
            <FilePond ref="myFilePond" name="myFilePond" class-name="filepond-custom" label-idle='上传本地文件'
                allow-multiple="true" v-bind:files="myFiles" styleButtonRemoveItemPosition="right" allowPaste="true"
                maxFiles="3" imagePreviewHeight="100" :style="{ display: pondVisiable ? 'block' : 'none' }" />
            <div class="chat-input">
                <Icon @click="showPicker" icon="fluent:emoji-add-24-regular" width="35" height="35"
                    style="color: #FFFFFF" />
                <Icon @click="appendFiles" icon="hugeicons:attachment-02" width="35" height="35"
                    style="color: #ffffff" />
                <input class="input-message" ref="messageInputRef" v-model="newMessage" @keydown.enter="sendMessage"
                    placeholder="Enter发送消息..." />
                <Icon @click="sendMessage" icon="fluent:send-20-regular" width="35" height="35"
                    style="color: #ffffff" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, nextTick, watch, onMounted } from 'vue';
import Cookies from 'js-cookie';
import { Icon } from '@iconify/vue';
import { Picker as EmojiPicker, EmojiIndex } from "emoji-mart-vue-fast/src";
import data from 'emoji-mart-vue-fast/data/all.json'
import emojiRegex from 'emoji-regex'
import vueFilePond from 'vue-filepond';
import FilePondPluginFileValidateType from 'filepond-plugin-file-validate-type/dist/filepond-plugin-file-validate-type.esm.js';
import FilePondPluginImagePreview from 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.esm.js';

import 'emoji-mart-vue-fast/css/emoji-mart.css'
import 'filepond/dist/filepond.min.css';
import 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.min.css';
import { formatDateTime, isImagePath, isVideoUrl, downloadFile } from '@/utils/commonUtil';
import ImagePreview from './ImagePreview.vue'
import { WebSocketService } from '@/utils/websocketService';

const FilePond = vueFilePond(FilePondPluginFileValidateType, FilePondPluginImagePreview);

export default defineComponent({
    name: 'ChatModal',
    components: {
        Icon,
        EmojiPicker,
        FilePond,
        ImagePreview
    },
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
        const currentTime = formatDateTime(new Date());
        const newMessage = ref('');
        const messages = ref([
            { text: '', time: '10:34:12', user: false, userName: '', fileName: '', files: [], notify: '阿达为你提供服务' },
            { text: '你好，请问有什么问题需要帮您解决？', time: '10:33:25', user: false, userName: '', files: [{ url: 'http://192.168.0.105/qi.png', name: '' }, { url: 'http://192.168.0.105/hahah.txt', name: 'hahah.txt' }, { url: 'http://192.168.0.105/hahah.txt', name: 'hahah.txt' }], notify: '' },
            { text: '你好，请问怎么充值', time: '10:33:55', user: true, userName: '', fileName: '', files: [{ url: 'http://192.168.0.105/q33.png', name: '' }, { url: 'http://192.168.0.105/dify.mp4', name: '' }, { url: 'http://192.168.0.105/hahah.txt', name: 'hahah.txt' }] },
            { text: '在系统后台选择充值服务，可以进行充值😊', time: '10:34:12', user: false, userName: '', fileName: '', files: [{ url: 'http://192.168.0.105/q33.png', name: '' }, { url: 'http://192.168.0.105/q33.png', name: '' }, { url: 'http://192.168.0.105/q.zip', name: 'q.zip' }], notify: '' },
            { text: '', time: '10:34:12', user: false, userName: '', fileName: '', files: [], notify: '您已离线<span @click="reconnect">重新连接</span>' },
        ]);

        const isEmojiPickerVisible = ref(false);
        const messageInputRef = ref<HTMLInputElement | null>(null);
        const focusInput = () => {
            if (messageInputRef.value) {
                messageInputRef.value.focus();
            }
        };

        let emojiIndex = new EmojiIndex(data, {
            emojisToShowFilter: (e: any) => {
                return true;
            },
            include: ['recent', 'smileys', 'people', 'nature', 'foods', 'activity'],
            exclude: [],
            custom: []
        });
        const pickerShow = ref(false);
        const showPicker = () => {
            pickerShow.value = !pickerShow.value;
            if (pondVisiable.value) {
                pondVisiable.value = false;
            }
        }

        const addEmoji = (emoji: any) => {
            newMessage.value += emoji.native;
            pickerShow.value = false;
        };

        const myFiles = ref<any>([]);

        const isImage = (url: string) => {
            return isImagePath(url)
        }

        const isVideo = (url: string) => {
            return isVideoUrl(url)
        }

        onMounted(() => {
            connect();
            nextTick(() => {
                focusInput();
            });
        });

        const websocketService = new WebSocketService();
        const connect = () => {
            const ukey = Cookies.get('bibirchat_ukey') ?? '0QsXuCkekVFG2cP';// for test
            const skey = Cookies.get('bibirchat_site_key') ?? 'pbAuq7PVr2gh2jp'; // for test
            const sserver = Cookies.get('bibirchat_sserver') ?? 'http://chat.local.com'; // for test
            console.log('uskey', ukey, skey);
            const url = sserver + '/clientchat?site_key=' + skey + '&room_key=' + ukey + '&client=1';
            websocketService.connect(url);
            websocketService.onMessage((data) => {
                console.log(data)
                messages.value.push({
                    text: data
                    , time: new Date().toLocaleString()
                    , user: false
                    , userName: ''
                    , fileName: ''
                    , files: []
                    , notify: ''
                });
                scrollToBottom();
            });
            websocketService.onClose(() => {
                console.log('断开连接');
            });
            websocketService.onSendError((data) => {
                console.log('发送失败', data);
            });
        };

        const sendMessage = () => {
            if (newMessage.value.trim()) {
                if (websocketService) {
                    const to_server = {
                        content: newMessage.value.trim()
                    };
                    websocketService.sendMessage(JSON.stringify(to_server));
                }
                messages.value.push({
                    text: newMessage.value,
                    time: new Date().toLocaleTimeString(),
                    user: true,
                    userName: '',
                    fileName: '',
                    files: [],
                    notify: ''
                });

                newMessage.value = '';
                scrollToBottom();
                focusInput();
            }
        };


        const handleKeydown = (event: KeyboardEvent) => {
            if (event.key === 'Escape') {
                closeModal();
            }
        }

        const closeModal = () => {
            emit('close');
        };

        const toggleEmojiPicker = () => {
            isEmojiPickerVisible.value = !isEmojiPickerVisible.value;
            if (pondVisiable.value) {
                pondVisiable.value = false;
            }
            // console.log("isEmojiPickerVisible:", isEmojiPickerVisible.value, "pondVisiable:", pondVisiable.value)
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
        const pondVisiable = ref(false);
        const previewImgSrc = ref('');
        const appendFiles = () => {
            pondVisiable.value = !pondVisiable.value;
            if (pickerShow.value) {
                pickerShow.value = false;
            }
            // console.log(myFiles.value)
            // nextTick(() => {
            //     // console.log(myFilePond)
            //     if (myFilePond.value) {
            //         myFilePond.value.click();
            //     }
            // });
            // console.log("isEmojiPickerVisible:", isEmojiPickerVisible.value, "pondVisiable:", pondVisiable.value)
        }
        const previewVisible = ref(false);
        const openImagePreview = (url: string) => {
            previewVisible.value = true;
            previewImgSrc.value = url;
            // console.log('previewVisible:', previewVisible.value, 'previewImgSrc:', previewImgSrc.value)
        }

        const toDownload = (url: string, name: string) => {
            downloadFile(url, name);
        }

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
            handleKeydown,
            emojiIndex,
            showPicker,
            pickerShow,
            myFiles,
            // handleFilePondInit,
            appendFiles,
            pondVisiable,
            isImage,
            isVideo,
            openImagePreview,
            previewImgSrc,
            previewVisible,
            toDownload
        };
    },
});
</script>

<style scoped>
.modal {
    position: fixed;
    z-index: 1;
    bottom: 20px;
    right: 20px;
    display: flex;
    justify-content: center;
    align-items: flex-end;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
    border-radius: 5px;
}

@media (min-width: 1024px) {
    .box {
        height: 600px;
        width: 90%;
    }
}

@media (min-width: 800px) {
    .box {
        height: 600px;
        width: 400px;
    }
}

.modal-content {
    padding: 10px;
    border: none;
    border-radius: 5px;
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
    padding: 5px;
    color: #f5f5f5;

    .chat-title {
        font-size: 20px;
    }

    .chat-time {
        font-size: 12px;
    }

    .chat-notify {
        font-size: 14px;
    }
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

    .message-notify {
        text-align: center;
        font-size: 11px;
        padding: 5px;
        color: grey;
    }

    .message-content {
        border-radius: 10px;
        padding: 0;
        margin: 0;
        width: auto;
        margin-bottom: 10px;
        border-radius: 10px;
        display: flex;

        .message-header {
            height: 35px;
            width: 35px;
            border-radius: 50%;
            background-color: #007BFF;
        }

        .message-main {
            flex-grow: 1;
            font-size: small;
            max-width: 92%;
            padding: 0 5px 0 5px;

            .message-text {
                background-color: rgba(145, 213, 242, 0.678);
                max-width: 100%;
                word-wrap: break-word;
                border-radius: 10px;
                padding: 5px;

                .content {
                    color: rgb(77, 70, 70);
                    text-align: left;
                }

                .message-time {
                    font-size: 10px;
                    color: #aaa;
                }
            }
        }
    }

}

.emoji-picker-container {
    position: absolute;
    bottom: 10%;
}

.input-message {
    font-size: small;
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

.emoji-text {
    font-size: 15px
}

.message-files {
    height: 100px;
    width: calc(33.33% - 0.5em);
    display: inline;
    list-style: none;
    padding: 0;
    margin: 0;

    img {
        height: 100px;
        width: 100px;
        padding: 3px;
    }


}
</style>
<style>
.filepond--credits {
    display: none;
}

.filepond-custom {
    /* height: 80px;
    width: 80px; */
    /* background-color: rgba(0, 123, 255, 0.8) */
    /* display: none; */
    /* position: absolute;
    bottom: -2px; */
    position: relative;
}

.filepond--item {
    width: calc(33.33% - 0.5em);
}

.filepond--root {
    /* max-height: 10em; */
}

.filepond--file-action-button {
    background-color: rgba(0, 0, 0, 0.5);
}

/* .filepond--root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial,
        sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';
} */
</style>