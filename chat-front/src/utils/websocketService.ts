import { sleep } from "./commonUtil";

// src/websocketService.ts
export class WebSocketService {
    private url: string | null = null;
    private socket: WebSocket | null = null;
    private timeoutId: number | null = null;
    private readonly TIMEOUT_DURATION = 3 * 60 * 1000; // 3分钟

    private onMessageCallback: ((data: any) => void) | null = null;
    private onCloseCallback: (() => void) | null = null;
    private onErrorCallback: ((error: any) => void) | null = null;
    private onSendErrorCallback: ((data: any) => void) | null = null;

    public connect(url: string): void {
        this.url = url;
        this.socket = new WebSocket(this.url);

        this.socket.onopen = () => {
            console.log('WebSocket connection opened.');
            this.resetTimeout();
        };

        this.socket.onmessage = (event) => {
            console.log('Message received:', event.data);
            if (this.onMessageCallback) {
                this.onMessageCallback(event.data);
            }
        };

        this.socket.onclose = () => {
            console.log('WebSocket connection closed.');
            this.clearTimeout();
            if (this.onCloseCallback) {
                this.onCloseCallback();
            }
        };

        this.socket.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.clearTimeout();
            if (this.onErrorCallback) {
                this.onErrorCallback(error);
            }
        };
    }

    private resetTimeout(): void {
        this.clearTimeout();
        this.timeoutId = window.setTimeout(() => {
            this.close();
        }, this.TIMEOUT_DURATION);
    }

    private clearTimeout(): void {
        if (this.timeoutId) {
            window.clearTimeout(this.timeoutId);
            this.timeoutId = null;
        }
    }

    public close(): void {
        if (this.socket) {
            this.socket.close();
            this.socket = null;
        }
    }

    public async sendMessage(message: string): Promise<void> {
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(message);
            this.resetTimeout(); // 发送消息时重置超时时间
        } else if (!this.socket || this.socket.readyState === WebSocket.CLOSED) {//关闭状态下发送时重新连接
            if (this.url) {
                console.info('WebSocket is closed. retry connect');
                this.connect(this.url)
                while (this.socket && this.socket.readyState !== WebSocket.OPEN) {
                    console.log();
                    await sleep(1000);
                }
                if (this.socket) {
                    this.socket.send(message);
                    this.resetTimeout(); // 发送消息时重置超时时间
                }
            }
        } else {
            console.error('WebSocket is not open.');
            if (this.onSendErrorCallback) {
                this.onSendErrorCallback(message);
            }
        }
    }

    public onMessage(callback: (data: any) => void): void {
        this.onMessageCallback = callback;
    }

    public onClose(callback: () => void): void {
        this.onCloseCallback = callback;
    }

    public onError(callback: (error: any) => void): void {
        this.onErrorCallback = callback;
    }

    public onSendError(callback: (data: any) => void): void {
        this.onSendErrorCallback = callback;
    }
}
