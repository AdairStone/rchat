// src/store/modules/messages.ts
import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import type { ChatMessageDto } from "@/utils/websocketService";

export const useMessagesStore = defineStore("messages", () => {
  // 使用 reactive 管理状态
  const messages = reactive<Record<string, ChatMessageDto[]>>({});
  const serverNotify = ref<ChatMessageDto>({} as ChatMessageDto);

  const addMessage = (message: ChatMessageDto) => {
    const { room_id } = message;
    if (message.to_server) {
      serverNotify.value = message;
    } else {
      if (!messages[room_id]) {
        messages[room_id] = [];
      }
      if (message.str_files) {
        message.files = JSON.parse(message.str_files);
      }
      messages[room_id].push(message);
    }
  };

  const resetMessage = (room_id: string) => {
    messages[room_id] = [];
  };

  const handleAddFirstMessage = (message: ChatMessageDto) => {
    const { room_id } = message;
    if (message.to_server) {
      serverNotify.value = message;
    } else {
      if (!messages[room_id]) {
        messages[room_id] = [];
      }
      messages[room_id].unshift(message);
    }
  };

  return {
    messages,
    serverNotify,
    addMessage,
    resetMessage,
    handleAddFirstMessage
  };
});
