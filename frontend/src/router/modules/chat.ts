import { $t } from "@/plugins/i18n";
const { VITE_HIDE_HOME } = import.meta.env;
const Layout = () => import("@/layout/index.vue");

export default {
  path: "/chat",
  name: $t("menus.chatList"),
  component: Layout,
  redirect: "/chat/list",
  meta: {
    icon: "ep:chat-line-round",
    title: $t("menus.chatList"),
    rank: 0
  },
  children: [
    {
      path: "/chat/list",
      name: "ChatList",
      component: () => import("@/views/website/ChatList.vue"),
      meta: {
        icon: "ep:chat-line-round",
        title: $t("menus.chatList"),
        showLink: VITE_HIDE_HOME === "true" ? false : true
      }
    }
    // ,
    // {
    //   path: "/chat/summary",
    //   name: "ChatSummary",
    //   component: () => import("@/views/website/ChatSummary.vue"),
    //   meta: {
    //     icon: "ep:data-line",
    //     title: $t("menus.chatSummary"),
    //     showLink: VITE_HIDE_HOME === "true" ? false : true
    //   }
    // }
  ]
} satisfies RouteConfigsTable;
