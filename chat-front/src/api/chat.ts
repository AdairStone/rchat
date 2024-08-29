// src/services/apiService.ts
import HttpClient from '@/utils/http';

const apiClient = new HttpClient(''); // 替换为你的API基础路径

export const loadMessages = async (payload: any) => {
    try {
        const response = await apiClient.post('/load/messages', payload);
        return response;
    } catch (error) {
        console.error('Error posting data:', error);
        throw error;
    }
};

export const loadSite = async (payload: any) => {
    try {
        const response = await apiClient.post('/load/site', payload);
        return response;
    } catch (error) {
        console.error('Error posting data:', error);
        throw error;
    }
};

