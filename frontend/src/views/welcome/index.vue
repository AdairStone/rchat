<script setup lang="ts">
import { onMounted, ref } from "vue";
import CodeBox from "./components/CodeBox.vue";
import { message } from "@/utils/message";
import { configWebsite, saveWebsite } from "@/api/website";
import { useChatSiteStore } from "@/store/modules/site";

const code = ref<string>("// Your initial code here...");
const siteStore = useChatSiteStore();

let siteData = ref({
  id: null,
  domain: null,
  create_at: null,
  position: null,
  site_key: null,
  status: null,
  title: null,
  update_at: null,
  user_id: null,
  welcome_slogan: null,
  version: null,
  script_home: null
});

const configSite = async () => {
  configWebsite()
    .then(res => {
      siteData.value = res.data;
      code.value =
        '\<scrit src="' +
        siteData.value.script_home +
        "/load/load.js?key=" +
        siteData.value.site_key +
        '"\>\</script\>';
      console.log(siteData.value);
      siteStore.setChatSite({
        site_id: siteData.value.id,
        site_key: siteData.value.site_key
      });
      console.log(code.value);
    })
    .catch(e => {
      console.log(e);
    });
};

onMounted(() => {
  configSite();
});

const activeStep = () => {
  if (siteData.value.status == "inited") {
    return 1;
  } else if (siteData.value.status == "confirmed") {
    return 2;
  } else {
    return 1;
  }
};

function onSubmit() {
  saveWebsite(siteData.value)
    .then(res => {
      siteData.value = res.data;
      message("保存成功", { type: "success" });
    })
    .catch(e => {
      message("保存失败：" + e);
    });
}

defineOptions({
  name: "Welcome",
  components: {}
});
</script>

<template>
  <div>
    <el-steps style="max-width: 700px" :active="activeStep()">
      <el-step title="第一步： 安装">
        <template #description>
          <div>
            请将下面代码添加到你的网页中,添加成功之后刷新该页面
            <code-box :initialCode="code"></code-box>
          </div>
        </template>
      </el-step>
      <el-step title="第二步： 配置">
        <template #description>
          <div>配置网站</div>
        </template>
      </el-step>
    </el-steps>
    <div>
      <el-form
        v-if="siteData.status === 'confirmed'"
        label-position="left"
        label-width="auto"
        :model="siteData"
        style="max-width: 500px"
      >
        <el-form-item label="网站编号">
          <el-input v-model="siteData.site_key" disabled />
        </el-form-item>
        <el-form-item label="聊天窗口标题">
          <el-input v-model="siteData.title" />
        </el-form-item>
        <el-form-item label="欢迎标语">
          <el-input v-model="siteData.welcome_slogan" />
        </el-form-item>
        <el-form-item label="显示位置">
          <el-input v-model="siteData.position" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="onSubmit">保存</el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>
