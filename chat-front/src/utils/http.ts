// src/utils/http.ts

import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse } from 'axios';

class HttpClient {
    private instance: AxiosInstance;

    constructor(baseURL: string) {
        this.instance = axios.create({
            baseURL,
            timeout: 10000, // 设置请求超时时间
            headers: {
                'Content-Type': 'application/json',
                // 你可以在这里添加其他默认的请求头
            },
        });

        this.instance.interceptors.request.use(
            (config: any) => {
                // 在请求发送之前可以进行一些处理，比如添加token
                // const token = localStorage.getItem('token');
                // if (token) {
                //   config.headers.Authorization = `Bearer ${token}`;
                // }
                return config;
            },
            (error) => {
                // 请求错误处理
                return Promise.reject(error);
            }
        );

        this.instance.interceptors.response.use(
            (response: AxiosResponse) => {
                // 对响应数据做处理
                return response.data;
            },
            (error) => {
                // 响应错误处理
                return Promise.reject(error);
            }
        );
    }

    public get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
        return this.instance.get(url, config);
    }

    public post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
        return this.instance.post(url, data, config);
    }

    public put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
        return this.instance.put(url, data, config);
    }

    public delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
        return this.instance.delete(url, config);
    }

    // 你可以添加其他请求方法（如 patch）或自定义方法
}

export default HttpClient;
