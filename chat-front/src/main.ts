import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'

const app = createApp(App, {
    compilerOptions: {
        isCustomElement: (tag: string) => tag === 'emoji-picker'
    }
})

app.use(createPinia())
app.mount('#adabibichatapp')
