// import { useChatWindowStore } from "@/store/modules/chatWindow";

import { useChatWindowStore } from "@/store/modules/chatWindow";

let isNotificationActive = false;
export function sendBrowserNotification(
  title: string,
  options?: NotificationOptions
) {
  if (Notification.permission === "granted") {
    console.log(
      "Notification granted isNotificationActive:",
      isNotificationActive
    );
    if (isNotificationActive == false) {
      isNotificationActive = true;
      const notification = new Notification(title, options);

      notification.onclick = event => {
        console.log("Notification clicked");
        event.preventDefault();
        isNotificationActive = false;

        // const url = options?.data?.url || "/#/chat/list";
        // if (window.location.href !== url) {
        //   window.open(url);
        // } else {
        //   window.focus();
        // }
      };
      notification.onclose = () => {
        console.log("Notification closed");
        isNotificationActive = false;
      };
      setTimeout(() => {
        if (isNotificationActive) {
          isNotificationActive = false;
        }
      }, 5000); // 这里假设通知在5秒后自动关闭
    }
  } else {
    console.log("Notification permission not granted.");
  }
}

export function sendNewMessageNotification(message: string) {
  let chatwindowStore = useChatWindowStore();
  console.log("chatwindowStore.isOpen", chatwindowStore.isOpen);
  if (chatwindowStore.isOpen == false) {
    sendBrowserNotification("你有新的消息", {
      body: message
    });
  }
}
