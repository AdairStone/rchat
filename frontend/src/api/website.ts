import { chatSiteType } from "@/store/types";
import { http } from "@/utils/http";

export type ConfigWebsiteResult = {
  status: number,
  request_id: string,
  success: boolean;
  detail: string;
  instance: string;
  data: {
    id: string,
    domain: string,
    create_at: Date,
    position: string,
    site_key: string,
    status: string,
    title: string;
    update_at: Date;
    user_id: number;
    welcome_slogan: number;
    version: number;
    script_home: string;
  };
};


export type RoomsResult = {
  status: number,
  request_id: string,
  success: boolean,
  detail: string,
  instance: string,
  data: RoomData,
};

export type RoomData = {
  data: Array<Room>,
  total: number,
};

export type Room = {
  id: string,
  create_at: Date,
  update_at: Date;
  version: number;
  room_key: string,
  room_site_id: string,
  status: string,
};

export type RoomMessagesResult = {
  status: number,
  request_id: string,
  success: boolean;
  detail: string;
  instance: string;
  data: Array<Room>;
};

export type Message = {
  id: string,
  create_at: Date,
  update_at: Date;
  version: number;
  user_id: string,
  status: string,
  room_id: string,
  name: string,
  reply_to_id: string,
  content: string,
  file_ids: Array<string>
};

export const configWebsite = (data?: object) => {
  return http.request<ConfigWebsiteResult>("post", "/api/service/config-site", { data });
};

export const saveWebsite = (data?: object) => {
  return http.request<ConfigWebsiteResult>("post", "/api/service/save-site", { data });
};

export const listSiteRooms = (data?: object) => {
  console.log(data);
  return http.get<RoomsResult, chatSiteType>("/api/service/list-rooms", {}, {params: data});
};

export const listMessages = (data?: object) => {
  console.log(data);
  return http.get<RoomsResult, chatSiteType>("/api/service/list-rooms", {}, {params: data});
};