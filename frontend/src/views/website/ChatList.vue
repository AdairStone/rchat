<script setup lang="ts">
import { Room, listSiteRooms } from "@/api/website";
import { $t } from "@/plugins/i18n";
import { useChatSiteStore } from "@/store/modules/site";
import { message } from "@/utils/message";
import { onMounted, ref } from "vue";
import ChatModal from "./ChatModal.vue";
import { websocketService } from "@/utils/websocketService";

const rooms = ref([]);
const roomKey = ref("");
const roomId = ref("");
const siteId = ref("");
const isOpen = ref(false);
const tableLoading = ref(false);
const queryParams = ref({
  total: 50,
  page: 1,
  pageSize: 10,
  roomKey: ""
});

const siteStore = useChatSiteStore();
async function getRoomList(data: { site_id: string }) {
  tableLoading.value = true;
  listSiteRooms(data)
    .then(res => {
      rooms.value = res.data.data;
      queryParams.value.total = res.data.total;
    })
    .catch(e => {
      message(
        $t("messages.chatSiteRoomListError") +
          ":" +
          JSON.stringify(e.response.data),
        {
          type: "error"
        }
      );
    })
    .finally(() => {
      tableLoading.value = false;
    });
}

function currentChange(val: Room) {
  handleClose();
  if (val) {
    roomKey.value = val.room_key;
    roomId.value = val.id;
    siteId.value = val.room_site_id;
    if (!isOpen.value) {
      isOpen.value = true;
      setTimeout(() => {
        document.querySelector(".modal")?.classList.add("open");
      }, 10);
      websocketService.joinRoom(val.id);
    }
  }
}

function handleClose() {
  document.querySelector(".modal")?.classList.remove("open");
  setTimeout(() => {
    isOpen.value = false;
  }, 300);
}

onMounted(() => {
  getRoomList({
    ...{ site_id: siteStore.chatSite.site_id },
    ...queryParams.value
  });
  websocketService.connect();
});

function handlePageChange(page, pageSize) {
  queryParams.value.page = page;
  queryParams.value.pageSize = pageSize;
  getRoomList({
    ...{ site_id: siteStore.chatSite.site_id },
    ...queryParams.value
  });
}
defineOptions({
  name: "ChatList",
  components: {
    ChatModal
  }
});
</script>
<template>
  <div>
    <el-pagination
      small
      layout="total, prev, pager, next, jumper"
      :total="queryParams.total"
      @change="handlePageChange"
    />
    <el-table
      :data="rooms"
      size="small"
      highlight-current-row
      :loading="tableLoading"
      @current-change="currentChange"
      @row-click="currentChange"
    >
      <el-table-column prop="id" label="会话编号" width="280" />
      <el-table-column prop="create_at" label="发起时间" width="180" />
      <el-table-column prop="update_at" label="最新时间" width="180" />
      <el-table-column prop="status" label="状态" width="80" />
      <el-table-column prop="room_key" label="客户编号" width="180" />
    </el-table>
    <ChatModal
      :isOpen="true"
      :roomKey="roomKey"
      :roomId="roomId"
      :siteId="siteId"
      @close="handleClose"
    />
  </div>
</template>
