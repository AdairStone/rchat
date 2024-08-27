<template>
  <div v-if="isOpen" class="modal box" @keydown="handleKeydown">
    <div class="modal-content">
      <span v-if="closeable" class="close" @click="closeModal">&times;</span>
      <div class="chat-header">
        <div class="chat-title">adabibi.com客服服务</div>
        <div class="chat-time">{{ currentTime }}</div>
        <div class="chat-notify">adabibi.com智能客服为您提供服务</div>
      </div>
      <div ref="chatMessage" class="chat-messages">
        <div v-for="(message, index) in messages" :key="index">
          <div
            v-if="message.notify && '' !== message.notify"
            class="message-notify"
          >
            {{ message.time }}
            <div v-html="message.notify" />
          </div>
          <div
            v-if="!message.notify || '' === message.notify"
            class="message-content"
          >
            <div
              class="message-header"
              :style="{ order: message.user ? '0' : '1' }"
            >
              <Icon
                v-if="!message.user"
                icon="ep:avatar"
                width="35"
                height="30"
                style="color: #ffffff"
              />
              <Icon
                v-if="message.user"
                icon="mdi:customer-service"
                width="35"
                height="35"
                style="color: #ffffff"
              />
            </div>
            <div
              class="message-main"
              :class="{ 'user-message': !message.user }"
            >
              <div
                v-if="
                  (message.text && '' !== message.text) ||
                  (message.files && message.files.length > 0)
                "
                class="message-text"
                :style="{ float: message.user ? 'left' : 'right' }"
              >
                <div style="display: inline-block">
                  <ul
                    v-if="message.files && message.files.length > 0"
                    class="message-files"
                  >
                    <li
                      v-for="i in message.files"
                      :key="i"
                      :style="{ float: message.user ? 'left' : 'right' }"
                    >
                      <img
                        v-if="isImage(i.url)"
                        :src="i.url"
                        @click="openImagePreview(i.url)"
                      />
                      <div v-if="isVideo(i.url)">
                        <video width="100" height="100" controls>
                          <source :src="i.url" type="video/mp4" />
                          hsla(160, 100%, 37%, 1 Your browser does not support
                          the video tag.
                        </video>
                      </div>
                      <div
                        v-if="!isVideo(i.url) && !isImage(i.url)"
                        @click="toDownload(i.url, i.file_name)"
                      >
                        <Icon
                          icon="mage:file-3"
                          width="75"
                          height="75"
                          style="color: #757070"
                        />
                        <div style="text-align: center">{{ i.file_name }}</div>
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
        <ImagePreview
          :visible="previewVisible"
          :src="previewImgSrc"
          @close="previewVisible = false"
        />
      </div>
      <div class="emoji-picker-container">
        <EmojiPicker
          v-if="pickerShow"
          :data="emojiIndex"
          emoji="grinning"
          :showPreview="false"
          :native="true"
          :skin="0"
          title="表情包"
          @select="addEmoji"
        />
      </div>
      <FilePond
        ref="myFilePond"
        name="myFilePond"
        class-name="filepond-custom"
        label-idle="上传本地文件"
        allow-multiple="true"
        styleButtonRemoveItemPosition="right"
        allowPaste="true"
        maxFiles="3"
        :files="pondFiles"
        imagePreviewHeight="100"
        :style="{
          display: pondVisiable || myFiles.length > 0 ? 'block' : 'none'
        }"
        server="/api/pub/file/upload"
        v-on:processfile="handleAddFile"
        v-on:removefile="handleRemoveFile"
        v-on:addfile="handleStartAddFile"
      />
      <div class="chat-input">
        <Icon
          icon="fluent:emoji-add-24-regular"
          width="35"
          height="35"
          style="color: #ffffff"
          @click="showPicker"
        />
        <Icon
          icon="hugeicons:attachment-02"
          width="35"
          height="35"
          style="color: #ffffff"
          @click="appendFiles"
        />
        <input
          ref="messageInputRef"
          v-model="newMessage"
          class="input-message"
          placeholder="Enter发送消息..."
          @keydown.enter="sendMessage"
        />
        <Icon
          icon="fluent:send-20-regular"
          width="35"
          height="35"
          style="color: #ffffff"
          @click="sendMessage"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  ref,
  nextTick,
  watch,
  onMounted,
  onUnmounted,
  onBeforeUpdate,
  onUpdated,
  computed
} from "vue";
import { Icon } from "@iconify/vue";
import { Picker as EmojiPicker, EmojiIndex } from "emoji-mart-vue-fast/src";
import data from "emoji-mart-vue-fast/data/all.json";
import vueFilePond from "vue-filepond";
import FilePondPluginFileValidateType from "filepond-plugin-file-validate-type/dist/filepond-plugin-file-validate-type.esm.js";
import FilePondPluginImagePreview from "filepond-plugin-image-preview/dist/filepond-plugin-image-preview.esm.js";

import "emoji-mart-vue-fast/css/emoji-mart.css";
import "filepond/dist/filepond.min.css";
import "filepond-plugin-image-preview/dist/filepond-plugin-image-preview.min.css";
import {
  formatDateTime,
  isImagePath,
  isVideoUrl,
  downloadFile
} from "@/utils/commonUtil";
import ImagePreview from "./ImagePreview.vue";
import { listMessages } from "@/api/website";
import { websocketService } from "@/utils/websocketService";
import { server } from "typescript";

const FilePond = vueFilePond(
  FilePondPluginFileValidateType,
  FilePondPluginImagePreview
);

export default defineComponent({
  name: "ChatModal",
  components: {
    Icon,
    EmojiPicker,
    FilePond,
    ImagePreview
  },
  props: {
    isOpen: {
      type: Boolean,
      required: true
    },
    roomKey: {
      type: String,
      required: true
    },
    roomId: {
      type: String,
      required: true
    },
    siteId: {
      type: String,
      required: true
    },
    closeable: {
      type: Boolean,
      required: false,
      default: true
    }
  },
  emits: ["close"],
  setup(props, { emit }) {
    const chatMessage = ref<HTMLElement | null>(null);
    const messageInputRef = ref<HTMLInputElement | null>(null);

    const currentTime = formatDateTime(new Date());
    const newMessage = ref("");
    // const messages = ref([]);

    const messages = computed(() => {
      console.log("listened", websocketService.messages);
      return websocketService.messages[props.roomId] || [];
    });

    watch(
      () => websocketService.messages,
      newMessages => {
        // console.log("message change", newMessages[props.roomId].files);
        console.log(
          "message change",
          newMessages[props.roomId],
          newMessages[props.roomId][newMessages[props.roomId].length - 1]
            ?.files,
          newMessages[props.roomId][newMessages[props.roomId].length - 1]?.files
            ?.length
        );
      },
      { deep: true } // 深度监听
    );

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
      include: ["recent", "smileys", "people", "nature", "foods", "activity"],
      exclude: [],
      custom: []
    });
    const pickerShow = ref(false);
    const showPicker = () => {
      pickerShow.value = !pickerShow.value;
      if (pondVisiable.value) {
        pondVisiable.value = false;
      }
    };

    const addEmoji = (emoji: any) => {
      newMessage.value += emoji.native;
      pickerShow.value = false;
    };

    const myFiles = ref<any>([]);
    const pondFiles = ref<any>([]);

    const isImage = (url: string) => {
      return isImagePath(url);
    };

    const isVideo = (url: string) => {
      return isVideoUrl(url);
    };
    const messageQuery = ref({
      page: 0,
      pageSize: 10,
      site_id: props.siteId,
      room_id: props.roomId,
      ts: null
    });
    const queryFull = ref(false);

    onMounted(() => {
      nextTick(() => {
        focusInput();
      });
      if (chatMessage.value) {
        console.log("register scroll", chatMessage.value);
        chatMessage.value.addEventListener("scroll", handleScroll);
      }
    });
    onUnmounted(() => {
      if (chatMessage.value) {
        console.log("remove scroll", chatMessage.value);
        chatMessage.value.removeEventListener("scroll", handleScroll);
      }
    });
    const handleScroll = () => {
      if (chatMessage.value) {
        const scrollTop = chatMessage.value.scrollTop;
        if (scrollTop === 0) {
          getMessages();
        }
      }
    };

    const handleKeydown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        closeModal();
      }
    };

    const closeModal = () => {
      emit("close");
    };

    const toggleEmojiPicker = () => {
      isEmojiPickerVisible.value = !isEmojiPickerVisible.value;
      if (pondVisiable.value) {
        pondVisiable.value = false;
      }
    };

    // 滚动到底部的方法
    const scrollToBottom = () => {
      if (!chatMessage.value) {
        return;
      }
      nextTick(() => {
        chatMessage.value!.scrollTop = chatMessage.value!.scrollHeight;
      });
    };

    let previousScrollHeight = 0;
    onBeforeUpdate(() => {
      // 在更新之前记录滚动条位置
      if (chatMessage.value) {
        previousScrollHeight = chatMessage.value.scrollHeight;
      }
    });

    onUpdated(() => {
      // 在更新之后调整滚动条位置
      if (chatMessage.value) {
        chatMessage.value.scrollTop +=
          chatMessage.value.scrollHeight - previousScrollHeight;
      }
    });

    watch(
      () => props.isOpen,
      (newValue, _) => {
        if (newValue) {
          scrollToBottom();
          focusInput();
        } else {
        }
      }
    );
    watch(
      () => props.roomId,
      async (newVal, oldVal) => {
        if (newVal !== "" && newVal !== oldVal) {
          // messages.value = [];
          messageQuery.value.page = 0;
          queryFull.value = false;
          closeModal();
          await getMessages();
          scrollToBottom();
        } else {
        }
      }
    );
    const getMessages = async () => {
      if (queryFull.value) {
        return;
      }
      messageQuery.value.page = messageQuery.value.page + 1;
      if (messageQuery.value.page === 1) {
        websocketService.resetMessage(props.roomId);
        messageQuery.value.ts = null;
      }
      if (messageQuery.value.page > 1 && messageQuery.value.ts == null) {
        messageQuery.value.page = 1;
      }
      await listMessages({
        ...messageQuery.value,
        ...{ room_id: props.roomId, site_id: props.siteId }
      }).then(res => {
        // console.log("getMessages", res);
        if (res.data && res.data?.data && res.data?.data.length > 0) {
          messageQuery.value.ts = res.data?.ts;
          let data = res.data?.data
            .map(message => {
              message.text = message.content;
              message.time = message.create_at;
              message.files = message.str_files
                ? JSON.parse(message.str_files)
                : [];
              message.user = message.user_id !== null;
              return message;
            })
            .sort((a, b) => {
              if (a.create_at > b.create_at) {
                return -1;
              } else {
                return 1;
              }
            });
          data.forEach(ms => {
            websocketService.handleAddFirstMessage(ms);
          });

          if (data.length < messageQuery.value.pageSize) {
            websocketService.handleAddFirstMessage({
              text: "",
              time: new Date().toLocaleString(),
              user: false,
              user_name: "",
              files: [],
              notify: "所有消息加载完成",
              room_id: props.roomId
            });
            queryFull.value = true;
          }
        } else {
          websocketService.handleAddFirstMessage({
            text: "",
            time: new Date().toLocaleString(),
            user: false,
            user_name: "",
            files: [],
            notify: "所有消息加载完成",
            room_id: props.roomId
          });
          queryFull.value = true;
        }
      });
    };
    const pondVisiable = ref(false);
    const previewImgSrc = ref("");
    const appendFiles = () => {
      pondVisiable.value = !pondVisiable.value;
      if (pickerShow.value) {
        pickerShow.value = false;
      }
    };
    const previewVisible = ref(false);
    const openImagePreview = (url: string) => {
      previewVisible.value = true;
      previewImgSrc.value = url;
    };

    const toDownload = (url: string, name: string) => {
      // downloadFile(url, name);
      window.open(url, "_blank");
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
        room_id: props.roomId
      };
      if (mess.text !== "" || (mess.files && mess.files.length > 0)) {
        websocketService.sendTalk(mess);
        newMessage.value = "";
        myFiles.value = [];
        pondFiles.value = [];
        pondVisiable.value = false;
        scrollToBottom();
      } else {
        return;
      }
    };

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

    return {
      currentTime,
      newMessage,
      messages,
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
      appendFiles,
      pondVisiable,
      isImage,
      isVideo,
      openImagePreview,
      previewImgSrc,
      previewVisible,
      toDownload,
      sendMessage,
      handleAddFile,
      handleRemoveFile,
      handleStartAddFile,
      pondFiles
    };
  }
});
</script>

<style scoped>
/* .modal {
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
} */

.modal {
  position: fixed;
  z-index: 9999;
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

/* Styles for screens wider than 600px */
@media (min-width: 400px) {
  .box {
    /* height: 600px; */
    width: 90%;
  }
}

/* Styles for screens wider than 800px */
@media (min-width: 450px) {
  .box {
    /* height: 100%; */
    width: 450px;
  }
}

/* Styles for screens wider than 800px */
@media (min-width: 850px) {
  .box {
    /* height: 600px; */
    width: 850px;
  }
}

/* @media (min-width: 1024px) {
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
} */

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
      background-color: #007bff;
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
  border-color: #00f2ff70;
  box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
  outline: none;
}

.chat-input button {
  margin-left: 10px;
  padding: 5px 10px;
  height: 40px;
  width: 60px;
  border: none;
  background-color: #007bff;
  color: white;
  border-radius: 4px;
  cursor: pointer;
}

.emoji-text {
  font-size: 15px;
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
