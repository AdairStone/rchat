import { defineStore } from "pinia";

export const useChatWindowStore = defineStore("chatWindow", {
  state: () => ({
    isOpen: false
  }),
  actions: {
    toggleOpen() {
      this.isOpen = !this.isOpen;
    },
    setOpen(value: boolean) {
      this.isOpen = value;
    }
  }
});
