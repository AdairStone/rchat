<template>
    <div v-if="isOpen" class="modal box" @keydown="handleKeydown">
        <div class="modal-content">
            <span v-if="closeable" class="close" @click="closeModal">
                <Icon icon="icon-park-outline:close-one" width="35" height="35" style="color: #ffffff" />
            </span>
            <div class="chat-header">
                <div class="chat-title">{{ siteInfo.title }}</div>
                <div class="chat-time">{{ siteInfo.start }}</div>
                <div class="chat-notify">{{ siteInfo.welcome_slogan }}</div>
            </div>
            <div ref="chatMessagesBody" class="chat-messages">
                <div v-for="(message, index) in messages" :key="index">
                    <div class="message-notify" v-if="message.notify && '' !== message.notify">
                        {{ message.time }}
                        <div v-html="message.notify"></div>
                    </div>
                    <div class="message-content" v-if="!message.notify || '' === message.notify">
                        <div class="message-header" :style="{ 'order': message.user ? '0' : '1' }">
                            <Icon v-if="message.user" icon="ep:avatar" width="35" height="30" style="color: #FFFFFF;" />
                            <Icon v-if="!message.user" icon="mdi:customer-service" width="35" height="35"
                                style="color: #FFFFFF" />
                        </div>
                        <div class="message-main" :class="{ 'user-message': !message.user }">
                            <div class="message-text" :style="{ 'float': message.user ? 'left' : 'right' }"
                                v-if="(message.text && message.text !== '') || (message.files && message.files.length > 0)">
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
                                                @click="toDownload(i.url, i.file_name)">
                                                <Icon icon="mage:file-3" width="75" height="75" style="color: #757070">
                                                </Icon>
                                                <div style="text-align: center;">{{ i.file_name }}</div>
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
            <FilePond ref="myFilePond" name="myFilePond" class-name="filepond-custom" label-idle="上传本地文件"
                allow-multiple="true" styleButtonRemoveItemPosition="right" allowPaste="true" maxFiles="3"
                :files="pondFiles" imagePreviewHeight="100" :style="{
                    display: pondVisiable || myFiles.length > 0 ? 'block' : 'none'
                }" server="/load/upload" v-on:processfile="handleAddFile" v-on:removefile="handleRemoveFile"
                v-on:addfile="handleStartAddFile" />
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
import { defineComponent, ref, nextTick, watch, onMounted, onUnmounted, onBeforeUpdate, onUpdated } from 'vue';
import Cookies from 'js-cookie';
import { Icon } from '@iconify/vue';
import { Picker as EmojiPicker, EmojiIndex } from "emoji-mart-vue-fast/src";
import data from 'emoji-mart-vue-fast/data/all.json'
import vueFilePond from 'vue-filepond';
import FilePondPluginFileValidateType from 'filepond-plugin-file-validate-type/dist/filepond-plugin-file-validate-type.esm.js';
import FilePondPluginImagePreview from 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.esm.js';

import 'emoji-mart-vue-fast/css/emoji-mart.css'
import 'filepond/dist/filepond.min.css';
import 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.min.css';
import { formatDateTime, isImagePath, isVideoUrl, downloadFile } from '@/utils/commonUtil';
import ImagePreview from './ImagePreview.vue'
import { WebSocketService } from '@/utils/websocketService';
import { loadMessages, loadSite } from '@/api/chat';

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
        const chatMessagesBody = ref<HTMLElement | null>(null);
        const messageInputRef = ref<HTMLInputElement | null>(null);

        const currentTime = formatDateTime(new Date());
        const newMessage = ref('');
        const messages = ref([
        ]);

        const isEmojiPickerVisible = ref(false);

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
        const pondFiles = ref<any>([]);

        const isImage = (url: string) => {
            return isImagePath(url)
        }

        const isVideo = (url: string) => {
            return isVideoUrl(url)
        }
        const registeredScroller = ref<any>(false);
        watch(props, (val) => {
            console.log("props changed", val);
            if (val.isOpen) {
                nextTick(() => {
                    if (chatMessagesBody.value && registeredScroller.value == false) {
                        console.log('register scroll', chatMessagesBody.value);
                        chatMessagesBody.value.addEventListener('scroll', handleScroll);
                        registeredScroller.value = true;
                    }
                    scrollToBottom();
                    focusInput();
                });
            }
        }, { deep: true });

        watch(messages, (val) => {
            // console.log("message changed", val)
        }, { immediate: true, deep: true });


        onMounted(() => {
            messages.value = [];
            queryCondition.value.page = 1;
            handleLoadSiteInfo();
            getMessages();
            connect();
        });

        onUnmounted(() => {
            if (chatMessagesBody.value) {
                console.log('remove scroll', chatMessagesBody.value);
                chatMessagesBody.value.removeEventListener('scroll', handleScroll);
            }
        });
        const handleScroll = () => {
            if (chatMessagesBody.value) {
                const scrollTop = chatMessagesBody.value.scrollTop;
                if (scrollTop === 0) {
                    getMessages();
                }
            }
        }
        const queryFull = ref(false);
        const queryCondition = ref({
            page: 1,
            page_size: 10,
            ts: null,
            site_key: Cookies.get('bibirchat_site_key') ?? 'pbAuq7PVr2gh2jp',
            room_key: Cookies.get('bibirchat_ukey') ?? '0QsXuCkekVFG2cP'
        });
        const getMessages = () => {
            // messages.value = [];
            if (queryFull.value == true) {
                return;
            }
            loadMessages(queryCondition.value).then(res => {
                let size = res.data?.data?.length;
                res.data?.data?.forEach((item: any) => {
                    item.text = item.content;
                    item.time = item.create_at;
                    item.files = item.str_files
                        ? JSON.parse(item.str_files)
                        : [];
                    item.user = item.user_id == null;

                    messages.value.unshift(item);
                });
                queryCondition.value.page++;
                queryCondition.value.ts = res.data?.ts;
                if (size && size < queryCondition.value.page_size) {
                    messages.value.unshift({
                        text: "",
                        time: new Date().toLocaleString(),
                        user: false,
                        user_name: "",
                        files: [],
                        notify: "所有消息加载完成"
                    });
                    queryFull.value = true;
                }
                // console.log("load messages:", res);
            }).catch(e => {
                console.log("error happended:", e)
            });
        }
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
                const jsonData = JSON.parse(data);
                if (jsonData.str_files) {
                    jsonData.files = JSON.parse(jsonData.str_files);
                }
                messages.value.push(jsonData);
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
            let mess = {
                text: newMessage.value,
                content: newMessage.value,
                time: new Date().toLocaleString(),
                user: true,
                user_name: "",
                str_files: JSON.stringify(myFiles.value),
                files: myFiles.value,
                notify: "",
                // room_id: props.roomId
            };
            if (mess.text !== "" || (mess.files && mess.files.length > 0)) {
                websocketService.sendMessage(JSON.stringify(mess));
                messages.value.push(mess);
                newMessage.value = "";
                myFiles.value = [];
                pondFiles.value = [];
                pondVisiable.value = false;
                scrollToBottom();
                focusInput();
                console.log(messages.value);
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


        // 滚动到底部的方法
        const scrollToBottom = () => {
            if (!chatMessagesBody.value) {
                return;
            }
            nextTick(() => {
                chatMessagesBody.value!.scrollTop = chatMessagesBody.value!.scrollHeight;
            });
        };

        let previousScrollHeight = 0;
        onBeforeUpdate(() => {
            // 在更新之前记录滚动条位置
            if (chatMessagesBody.value) {
                previousScrollHeight = chatMessagesBody.value.scrollHeight;
            }
        });

        onUpdated(() => {
            // 在更新之后调整滚动条位置
            if (chatMessagesBody.value) {
                chatMessagesBody.value.scrollTop +=
                    chatMessagesBody.value.scrollHeight - previousScrollHeight;
            }
        });


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

        const handleAddFile = (error, file: File) => {
            // console.log("file add:", error, file);
            if (error === null && file !== undefined) {
                let file2 = JSON.parse(file.serverId);
                let file3 = file2?.data?.entry?.files[0];
                file3.id = file.id;
                console.log("after:", file3, myFiles.value);
                myFiles.value.push(file3);
            }
        };

        const handleRemoveFile = (e, file: File) => {
            // console.log("file remove:", e, file);
            // let file2 = JSON.parse(file.serverId);
            let fileId = file?.id;
            // let file3 = file2?.data?.entry?.files[0];
            if (fileId && myFiles && myFiles.value.length > 0) {
                // console.log("remove file3:", fileId, myFiles.value);
                console.log(
                    "remove file3 after:",
                    myFiles.value.filter(f => f.id !== fileId)
                );
                myFiles.value = myFiles.value.filter(f => f.id !== fileId);
            }
        };

        const handleStartAddFile = (e, file: File) => {
            let file2 = JSON.parse(file.serverId);
            let file3 = file2?.data?.entry?.files[0];
            // console.log("file start:", e, file, file3, myFiles.value);
        };

        const siteInfo = ref({
            position: '',
            welcome_slogan: '',
            title: '',
            start: '',
        });

        const handleLoadSiteInfo = () => {
            loadSite({ "site_key": queryCondition.value.site_key }).then(res => {
                console.log("loadSiteInfo:", res);
                siteInfo.value = res.data
            }).catch(e => {
                console.log(e)
            })
        }

        return {
            currentTime,
            newMessage,
            messages,
            sendMessage,
            closeModal,
            toggleEmojiPicker,
            addEmoji,
            chatMessagesBody,
            scrollToBottom,
            messageInputRef,
            focusInput,
            handleKeydown,
            emojiIndex,
            showPicker,
            pickerShow,
            myFiles,
            pondFiles,
            // handleFilePondInit,
            appendFiles,
            pondVisiable,
            isImage,
            isVideo,
            openImagePreview,
            previewImgSrc,
            previewVisible,
            toDownload,
            handleAddFile,
            handleRemoveFile,
            handleStartAddFile,
            handleLoadSiteInfo,
            siteInfo
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
    width: calc(33.33% - 10em);
    display: inline;
    list-style: none;
    padding: 0;
    margin: 0;

    img {
        height: 100px;
        width: 97px;
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