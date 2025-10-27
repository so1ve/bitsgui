import { createRouter, createWebHashHistory } from "vue-router";

import LoginView from "./views/Login.vue";
import StatusView from "./views/Status.vue";

const routes = [
	{ path: "/", component: LoginView },
	{ path: "/status", component: StatusView },
];

export const router = createRouter({
	history: createWebHashHistory(),
	routes,
});
