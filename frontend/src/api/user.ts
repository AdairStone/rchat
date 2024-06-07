import { http } from "@/utils/http";

export type UserResult = {
  success: boolean;
  data: {
    entry: {
      id: string,
      name: string;
      roles: Array<string>;
    },
    access_token: string;
    refresh_token: string;
    expires_in: number;
  };
};

export type RefreshTokenResult = {
  success: boolean;
  data: {
    access_token: string;
    expires_in: number;
  };
};

/** 登录 */
export const getLogin = (data?: object) => {
  return http.request<UserResult>("post", "/api/auth/login", { data });
};

/** 刷新`token` */
export const refreshTokenApi = (data?: object) => {
  return http.request<RefreshTokenResult>("get", "/api/auth/refresh");
};
