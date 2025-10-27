import { createApp } from "vue";
import App from "./App.vue";
import router from"./router";
import 'element-plus/theme-chalk/dark/css-vars.css'
import {useDark} from "@vueuse/core";
const app=createApp(App)
app.use(router).mount("#app");
useDark()