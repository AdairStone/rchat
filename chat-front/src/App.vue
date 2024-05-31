<template>
  <div id="app">
    <div v-if="!isChatOpen" class="floating-button" ref="floatingButton">
      <button @click="openModal">💬</button>
    </div>
    <ChatModal :isOpen="isChatOpen" @close="closeModal" />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import ChatModal from './components/ChatModal.vue';

export default defineComponent({
  name: 'App',
  components: {
    ChatModal,
  },
  setup() {
    const isChatOpen = ref(false);

    const openModal = () => {
      isChatOpen.value = true;
      setTimeout(() => {
        document.querySelector('.modal')?.classList.add('open');
      }, 10); // Small delay to ensure the class is added after the modal is displayed
    };

    const closeModal = () => {
      document.querySelector('.modal')?.classList.remove('open');
      setTimeout(() => {
        isChatOpen.value = false;
      }, 300); // Match the transition duration
    };
    // Draggable button logic
    const floatingButton = ref<HTMLElement | null>(null);
    return {
      isChatOpen,
      openModal,
      closeModal,
      floatingButton,
      // startDrag,
    };
  },
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

.floating-button {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
  cursor: move;
}

.floating-button button {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  border: none;
  background-color: #007bff;
  color: white;
  font-size: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  cursor: pointer;
}

.floating-button button:hover {
  background-color: #0056b3;
}

/* 滚动条整体样式 */
::-webkit-scrollbar {
  width: 10px;
  /* 滚动条宽度 */
}

/* 滚动条滑块样式 */
::-webkit-scrollbar-thumb {
  background-color: #888;
  /* 滑块颜色 */
  border-radius: 5px;
  /* 滑块圆角 */
}

/* 滚动条滑块悬停样式 */
::-webkit-scrollbar-thumb:hover {
  background-color: #555;
  /* 悬停时滑块颜色 */
}
</style>
