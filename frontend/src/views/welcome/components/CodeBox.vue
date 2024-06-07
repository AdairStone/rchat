<template>
  <div>
    <button @click="copyCode">
      <el-icon><DocumentCopy /></el-icon>复制代码
    </button>
    <pre><code>{{ code }}</code></pre>
  </div>
</template>

<script setup lang="ts">
import { message } from "@/utils/message";
import { ref, watch } from "vue";

const props = defineProps<{
  initialCode: string;
}>(); // 定义props接口

const code = ref<string>(props.initialCode);

const copyCode = async () => {
  try {
    await navigator.clipboard.writeText(code.value);
    message("已复制到剪切板", { type: "success" });
  } catch (error) {
    message("复制失败", { type: "error" });
  }
};

watch(
  () => props.initialCode,
  newValue => {
    code.value = newValue;
  }
);
</script>

<style scoped>
/* 可以根据需要添加样式 */
pre {
  background-color: #f4f4f4;
  padding: 10px;
  border-radius: 5px;
  overflow-x: wrap;
}
</style>
