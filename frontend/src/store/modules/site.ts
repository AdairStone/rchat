import { defineStore } from 'pinia';

// 定义类型
export type ChatSiteType = {
  site_id?: string;
  site_key?: string;
};
const CHAT_SITE_KEY = "chat-site-key";
function loadState(): ChatSiteType {
  const state = localStorage.getItem(CHAT_SITE_KEY);
  return state ? JSON.parse(state) : {};
}

// 定义 store
export const useChatSiteStore = defineStore(CHAT_SITE_KEY, {
  state: () => ({
    chatSite: loadState(),
  }),
  actions: {
    setChatSite(site: ChatSiteType) {
      this.chatSite = site;
      localStorage.setItem(CHAT_SITE_KEY, JSON.stringify(site));
    },
  },
});
