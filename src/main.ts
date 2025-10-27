import { attachConsole } from "@tauri-apps/plugin-log";
import { createApp } from "vue";

import App from "./App.vue";
import { initBitsrun, initStore } from "./api";
import { router } from "./router";

import "./style.css";

await initBitsrun();
await initStore();

const _detach = await attachConsole();

createApp(App).use(router).mount("#app");
