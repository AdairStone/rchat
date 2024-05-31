// stores/messageStore.ts
import { defineStore } from 'pinia';

interface Message {
    text: string;
    time: string;
    user: boolean;
}

interface State {
    messages: Message[];
}

export const useMessageStore = defineStore('message', {
    state: (): State => ({
        messages: [],
    }),
    actions: {
        addMessage(message: Message) {
            this.messages.push(message);
        },
    },
    getters: {
        allMessages: (state) => state.messages,
    },
});
