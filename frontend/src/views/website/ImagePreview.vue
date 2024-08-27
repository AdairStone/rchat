<template>
  <div v-if="visible" class="overlay">
    <div
      ref="imageContainer"
      class="image-container"
      :style="{
        transform: `translate(${translateX}px, ${translateY}px) scale(${scale})`
      }"
      @click.stop
      @wheel="handleWheel"
      @mousedown="handleMouseDown"
    >
      <img ref="imageRef" :src="src" @load="handleImageLoad" />
    </div>
    <div class="controls">
      <button @click="zoomIn">+</button>
      <button @click="zoomOut">-</button>
      <button @click="close">X</button>
    </div>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  ref,
  onMounted,
  onBeforeUnmount,
  nextTick
} from "vue";

export default defineComponent({
  name: "ImagePreview",
  props: {
    src: {
      type: String,
      required: true
    },
    visible: {
      type: Boolean,
      required: true
    }
  },
  emits: ["close"],
  setup(_, { emit }) {
    const scale = ref(1);
    const translateX = ref(0);
    const translateY = ref(0);
    const isDragging = ref(false);
    const startX = ref(0);
    const startY = ref(0);
    const imageContainer = ref<HTMLDivElement | null>(null);
    const imageRef = ref<HTMLImageElement | null>(null);

    const close = () => {
      emit("close");
    };

    const zoomIn = () => {
      scale.value += 0.1;
    };

    const zoomOut = () => {
      if (scale.value > 0.1) {
        scale.value -= 0.1;
      }
    };

    const handleWheel = (event: WheelEvent) => {
      if (event.deltaY > 0) {
        zoomOut();
      } else {
        zoomIn();
      }
      event.preventDefault();
    };

    const handleMouseDown = (event: MouseEvent) => {
      isDragging.value = true;
      startX.value = event.clientX - translateX.value;
      startY.value = event.clientY - translateY.value;
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    };

    const handleMouseMove = (event: MouseEvent) => {
      if (isDragging.value) {
        translateX.value = event.clientX - startX.value;
        translateY.value = event.clientY - startY.value;
      }
    };

    const handleMouseUp = () => {
      isDragging.value = false;
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    const handleImageLoad = () => {
      if (imageRef.value && imageContainer.value) {
        const imgWidth = imageRef.value.naturalWidth;
        const imgHeight = imageRef.value.naturalHeight;
        const containerWidth = imageContainer.value.clientWidth;
        const containerHeight = imageContainer.value.clientHeight;

        const widthRatio = containerWidth / imgWidth;
        const heightRatio = containerHeight / imgHeight;

        scale.value = Math.min(widthRatio, heightRatio);
      }
    };

    onMounted(() => {
      if (imageContainer.value) {
        imageContainer.value.addEventListener("mousedown", handleMouseDown);
      }
    });

    onBeforeUnmount(() => {
      if (imageContainer.value) {
        imageContainer.value.removeEventListener("mousedown", handleMouseDown);
      }
    });

    return {
      scale,
      translateX,
      translateY,
      close,
      zoomIn,
      zoomOut,
      handleWheel,
      handleMouseDown,
      handleMouseMove,
      handleMouseUp,
      handleImageLoad,
      imageContainer,
      imageRef
    };
  }
});
</script>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.image-container {
  position: relative;
  /* max-width: 90%;
    max-height: 90%; */
  width: 95vw;
  cursor: grab;
  overflow: hidden;
}

.image-container img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.controls {
  position: absolute;
  bottom: 10px;
  right: 10px;
  display: flex;
  gap: 10px;
}

.controls button {
  padding: 10px;
  background: rgba(255, 255, 255, 0.8);
  border: none;
  cursor: pointer;
}

.image-container:active {
  cursor: grabbing;
}
</style>
