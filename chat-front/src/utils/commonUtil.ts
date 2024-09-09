export const isImagePath = (path: string): boolean => {
    const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.svg', '.webp'];
    const extension = path.split('.').pop()?.toLowerCase();
    return extension ? imageExtensions.includes(`.${extension}`) : false;
};

export const formatDateTime = (date: Date): string => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // 月份从0开始，所以加1
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
};

const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'mkv', 'webm', 'flv', 'ogg'];

export const isVideoUrl = (url: string) => {
    try {
        const urlObj = new URL(url);
        const pathname = urlObj.pathname;
        const extension = pathname.split('.').pop()?.toLowerCase();
        return videoExtensions.includes(extension || '');
    } catch (error) {
        console.error('Invalid URL:', error);
        return false;
    }
}

export function downloadFile(url: string, filename: string) {
    fetch(url)
        .then(response => response.blob())
        .then(blob => {
            const link = document.createElement('a');
            const objectURL = URL.createObjectURL(blob);
            link.href = objectURL;
            link.download = filename;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(objectURL);
        })
        .catch(error => console.error('Download error:', error));
}

// 定义 sleep 函数
export const sleep = (ms: number) => {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export function playSound(src: string): void {
    const audio = new Audio(src);
    audio.play().catch(error => {
        console.error("播放音频时出错:", error);
    });
}


