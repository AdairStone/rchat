import { onUnmounted } from "vue";
import { getToken } from "./auth";
import { useChatSiteStore } from "@/store/modules/site";
import { sendNewMessageNotification } from "./browserNotify";
import { useMessagesStore } from "@/store/modules/messages";

interface ChatNotifyMessageDto {
  total_unread: number; // 对应 Rust 中的 i32
  new_message: boolean; // 对应 Rust 中的 bool
  message_counts: { [key: string]: number }; // 对应 Rust 中的 HashMap<String, i32>
}
interface ChatMessageFileDto {
  url: string;
  name: string;
}

export interface ChatMessageDto {
  text: string;
  time: string;
  user: boolean;
  user_name?: string;
  str_files?: string;
  files: ChatMessageFileDto[];
  notify: string;
  to_server?: boolean;
  message?: ChatNotifyMessageDto;
  room_id?: string;
}

// src/websocketService.ts
class WebsocketService {
  private url: string | null = null;
  private socket: WebSocket | null = null;
  private timeoutId: number | null = null;
  private intervalId: number | null = null;

  private readonly TIMEOUT_DURATION = 7 * 24 * 60 * 60 * 1000; // 3分钟
  private readonly CHECK_INTERVAL = 10 * 1000; // 1 minute

  private messagesStore = useMessagesStore();
  constructor() {
    try {
      // this.connect();
    } catch (e) {
      console.error(e);
    }
  }

  public connect(): void {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      return;
    }
    console.log("connecting...");
    const siteStore = useChatSiteStore();
    const token = getToken()?.accessToken;
    const url =
      "/ws/chat?access_token=" +
      token +
      "&client=0&room_key=" +
      "" +
      "&site_key=" +
      siteStore.chatSite.site_key;
    this.url = url;
    this.socket = new WebSocket(this.url);

    this.socket.onopen = () => {
      console.log("WebSocket connection opened.");
      this.resetTimeout();
    };

    this.socket.onmessage = event => {
      console.log("Message received:", event.data);
      const message = JSON.parse(event.data) as ChatMessageDto;
      this.handleMessage(message);
    };

    this.socket.onclose = () => {
      console.log("WebSocket connection closed.");
      this.clearTimeout();
      if (this.socket) {
        this.socket.close();
      }
    };

    this.socket.onerror = error => {
      console.error("WebSocket error:", error);
      this.clearTimeout();
    };
    this.startConnectionCheck();
  }

  public handleMessage(message: ChatMessageDto) {
    // console.log("handleMessage:", message);
    if (message.to_server) {
      this.messagesStore.serverNotify = message;
      sendNewMessageNotification(JSON.stringify(message));
    } else {
      this.messagesStore.addMessage(message);
    }
  }

  public handleAddFirstMessage(message: ChatMessageDto) {
    this.messagesStore.handleAddFirstMessage(message);
  }

  public resetMessage(room_id: string) {
    this.messagesStore.resetMessage(room_id);
  }

  private resetTimeout(): void {
    this.clearTimeout();
    this.timeoutId = window.setTimeout(() => {
      this.close();
    }, this.TIMEOUT_DURATION);
  }

  private clearTimeout(): void {
    if (this.timeoutId) {
      window.clearTimeout(this.timeoutId);
      this.timeoutId = null;
    }
  }

  private startConnectionCheck(): void {
    this.intervalId = window.setInterval(() => {
      console.log("check connection");
      if (this.socket && this.socket.readyState === WebSocket.CLOSED) {
        console.info("WebSocket is closed. Attempting to reconnect...");
        this.reconnect();
      }
    }, this.CHECK_INTERVAL);
  }

  private stopConnectionCheck(): void {
    if (this.intervalId) {
      window.clearInterval(this.intervalId);
      this.intervalId = null;
    }
  }

  public close(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
      console.log("close websocket");
    }
    this.stopConnectionCheck();
  }

  public async sendMessage(message: string): Promise<void> {
    try {
      if (this.socket && this.socket.readyState === WebSocket.OPEN) {
        this.socket.send(message);
        this.resetTimeout(); // 发送消息时重置超时时间
      } else if (!this.socket || this.socket.readyState === WebSocket.CLOSED) {
        console.info("WebSocket is closed. Attempting to reconnect...");
        await this.reconnect();
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
          this.socket.send(message);
          this.resetTimeout(); // 发送消息时重置超时时间
        } else {
          throw new Error("WebSocket could not be opened after reconnection.");
        }
      } else {
        throw new Error("WebSocket is not open.");
      }
    } catch (error) {
      console.error("Failed to send message:", error);
      throw error; // 将错误抛出，方便调用方处理
    }
  }

  private async reconnect(): Promise<void> {
    if (!this.socket || this.socket.readyState === WebSocket.CLOSED) {
      this.connect();
      const timeout = 10000; // 5秒超时
      const interval = 100; // 每100ms检查一次
      let elapsed = 0;
      while (this.socket && this.socket.readyState !== WebSocket.OPEN) {
        if (elapsed >= timeout) {
          throw new Error("WebSocket reconnection timed out.");
        }
        await this.sleep(interval);
        elapsed += interval;
      }
    }
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  public async joinRoom(room_id: string): Promise<void> {
    const message = "/join " + room_id;
    return await this.sendMessage(message);
  }

  public async sendTalk(message: ChatMessageDto): Promise<void> {
    this.handleMessage(message);
    this.sendMessage(JSON.stringify(message));
  }

  // loadMessagesFromStorage() {
  //   Object.keys(localStorage).forEach(key => {
  //     if (key.startsWith("room_")) {
  //       const roomId = key.replace("room_", "");
  //       const messages = JSON.parse(localStorage.getItem(key) || "[]");
  //       this.messages[roomId] = messages;
  //     }
  //   });
  // }
}
let websocketService: WebsocketService | null = null;

export function useWebsocketService(): WebsocketService {
  if (!websocketService) {
    websocketService = new WebsocketService();
  }
  // onUnmounted(() => {
  //   websocketService?.close();
  // });
  return websocketService;
}
