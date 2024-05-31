import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import VueDevTools from 'vite-plugin-vue-devtools'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    VueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
    proxy: {
      // 将本地开发服务器的 /api 路径代理到 http://localhost:3000
      '/socket': {
        target: 'ws://127.0.0.1:8080/ws',
        changeOrigin: true,
        ws: true, // 启用 Websocket 代理
      },
    },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    rollupOptions: {
      output: {
        manualChunks: {
          vue: ['vue'],
        },
        entryFileNames: 'chat.js', // 输出文件名
        chunkFileNames: '[name]-[hash].js', // 其他 chunk 文件名
        assetFileNames: '[name]-[hash][extname]', // 其他静态资源文件名
      },
    },
  },
  // server: {
  //   proxy: {
  //     // 代理 '/api' 到 'http://localhost:3000/api'
  //     '/ws': {
  //       target: 'http://localhost:8080',
  //       changeOrigin: true,
  //       rewrite: (path) => path.replace(/^\/api/, '')
  //     }
  //   }
  // }
})
