/**
 * main.js
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from './App.vue'
import 'vue-toast-notification/dist/theme-sugar.css';

// Composables
import { createApp } from 'vue'

// Plugins
import { registerPlugins } from '@/plugins'
import ToastPlugin from "vue-toast-notification";
import {createPinia} from "pinia";


const app = createApp(App)

const pinia = createPinia()
app.use(pinia)


registerPlugins(app)

window.baseURL = "http://localhost:3000"

app.use(ToastPlugin, {
    position: 'top-right',
});

app.mount('#app')
