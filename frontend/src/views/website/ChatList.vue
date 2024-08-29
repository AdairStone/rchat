<script setup lang="ts">
import { Room, listSiteRooms } from "@/api/website";
import { $t } from "@/plugins/i18n";
import { useChatSiteStore } from "@/store/modules/site";
import { message } from "@/utils/message";
import { computed, onMounted, ref, watch } from "vue";
import ChatModal from "./ChatModal.vue";
import { useWebsocketService } from "@/utils/websocketService";
import { useMessagesStore } from "@/store/modules/messages";
import { useChatWindowStore } from "@/store/modules/chatWindow";
const websocketService = useWebsocketService();
const chatwindowStore = useChatWindowStore();
const rooms = ref([]);
const roomKey = ref("");
const roomId = ref("");
const siteId = ref("");
const isOpen = computed(() => chatwindowStore.isOpen);
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

async function currentChange(val: Room) {
  if (val && roomId.value !== val.id) {
    roomKey.value = val.room_key;
    roomId.value = val.id;
    siteId.value = val.room_site_id;
    console.log("isOpen", isOpen.value);
    chatwindowStore.setOpen(true);
    setTimeout(() => {
      document.querySelector(".modal")?.classList.add("open");
    }, 10);
    websocketService.joinRoom(val.id);
  }
}

function handleClose() {
  console.log("handleClose isOpen 1", isOpen.value);
  if (chatwindowStore.isOpen) {
    console.log("handleClose isOpen 2", isOpen.value);
    document.querySelector(".modal")?.classList.remove("open");
    chatwindowStore.setOpen(false);
    websocketService.joinRoom("00000000-0000-0000-0000-000000000000");
    roomId.value = "";
  }
}
const messagesStore = useMessagesStore();
const roomMessages = computed(() => {
  return messagesStore.serverNotify?.message?.message_counts || [];
});

watch(
  () => messagesStore.serverNotify,
  (new_notify, old_notify) => {
    // getRoomList({
    //   ...{ site_id: siteStore.chatSite.site_id },
    //   ...queryParams.value
    // });
  }
  // { deep: true } // 深度监听
);

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
      <el-table-column prop="id" label="会话编号" width="300">
        <template v-slot="{ row }">
          <el-badge :value="roomMessages[row.id] || ''" class="item">
            <span>{{ row.id }}</span>
          </el-badge>
        </template>
      </el-table-column>
      <el-table-column prop="create_at" label="发起时间" width="180" />
      <el-table-column prop="update_at" label="最新时间" width="180" />
      <el-table-column prop="status" label="状态" width="80" />
      <el-table-column prop="room_key" label="客户编号" width="180" />
      <el-table-column prop="ip" label="请求IP" width="180">
        <template v-slot="{ row }">
          <div>
            {{ row.client_info ? JSON.parse(row.client_info)?.ip : "" }}
          </div>
        </template>
      </el-table-column>
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

<style lang="scss">
.item {
  margin-top: 10px;
  margin-right: 40px;
}
</style>
