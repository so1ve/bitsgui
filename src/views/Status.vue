<script setup lang="ts">
import { useIntervalFn } from "@vueuse/core";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { toast } from "vue-sonner";

import { checkStatus, isLoggedIn, logout, setLoggedIn } from "../api";
import AppFooter from "../components/AppFooter.vue";
import { state } from "../state";
import type { SrunLoginState, SrunLoginStateLoggedIn } from "../types";
import { formatFlux, humanizeDuration } from "../utils";

const router = useRouter();
const data = ref(null as null | SrunLoginState<SrunLoginStateLoggedIn>);
const loading = ref(false);

async function updateStatusOrRedirect() {
	const response = await checkStatus();
	const loggedIn = await isLoggedIn();
	await setLoggedIn(loggedIn);

	if (!loggedIn) {
		router.push("/");
	} else if (response.success) {
		data.value = response.data as any;
	}
}

useIntervalFn(updateStatusOrRedirect, 3000, {
	immediate: true,
	immediateCallback: true,
});

async function handleLogout() {
	state.manualLogout = true;
	loading.value = true;
	const response = await logout(state.credentials.username);

	if (response.success) {
		toast.success("注销成功！");
	} else {
		loading.value = false;
		toast.error(`注销失败：${response.error}`);
	}

	updateStatusOrRedirect();
}
</script>

<template>
	<div>
		<div v-if="data" class="flex flex-col gap-4">
			<h2 class="text-2xl font-semibold text-center">登录状态</h2>
			<div class="card bg-gray-100/50 shadow-sm">
				<table class="table table-sm">
					<tbody>
						<tr>
							<td>用户名</td>
							<td>{{ data.user_name }}</td>
						</tr>
						<tr>
							<td>IP</td>
							<td>{{ data.online_ip }}</td>
						</tr>
						<tr>
							<td>已用流量</td>
							<td>
								{{ formatFlux(data.sum_bytes) }}
							</td>
						</tr>
						<tr>
							<td>已用时长</td>
							<td>{{ humanizeDuration(data.sum_seconds * 1000) }}</td>
						</tr>
						<tr>
							<td>账户余额</td>
							<td>{{ data.user_balance }}</td>
						</tr>
					</tbody>
				</table>
			</div>
			<button
				class="btn w-full"
				:class="{
					'btn-error': !loading,
					'btn-disabled': loading,
				}"
				:disabled="loading"
				@click="handleLogout"
			>
				<span v-if="loading" class="loading loading-spinner" />
				注销
			</button>
		</div>
		<AppFooter />
	</div>
</template>
