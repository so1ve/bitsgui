<script setup lang="ts">
import { useIntervalFn } from "@vueuse/core";
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import { toast } from "vue-sonner";

import { isLoggedIn, login, setCredentials } from "../api";
import { state } from "../state";

const router = useRouter();
const loading = ref(true);

useIntervalFn(checkStatusAndRedirect, 2000, {
	immediate: true,
	immediateCallback: true,
});

async function checkStatusAndRedirect() {
	const loggedIn = await isLoggedIn();
	if (loggedIn) {
		router.push("/status");
	}
}

watch(
	() => state.credentials.rememberMe,
	(newVal) => {
		if (newVal === false && state.credentials.autoLogin === true) {
			state.credentials.autoLogin = false;
		}
	},
);

watch(
	() => state.credentials.autoLogin,
	(newVal) => {
		if (newVal === true && state.credentials.rememberMe === false) {
			state.credentials.rememberMe = true;
		}
	},
);

if (state.firstOpen) {
	state.firstOpen = false;

	if (state.credentials.autoLogin && !state.manualLogout) {
		await login(state.credentials);
		await checkStatusAndRedirect();
	} else {
		loading.value = false;
	}
} else {
	loading.value = false;
}

async function handleLogin() {
	loading.value = true;
	if (state.credentials.rememberMe) {
		await setCredentials(state.credentials);
	}
	const response = await login(state.credentials);
	if (response.success) {
		await checkStatusAndRedirect();
		toast.success("登录成功！");
	} else {
		loading.value = false;
		toast.error(`登录失败：${response.error}`);
	}
}
</script>

<template>
	<div>
		<h2 class="text-2xl font-semibold mb-2 text-center">登录</h2>
		<form class="flex flex-col gap-4" @submit.prevent="handleLogin">
			<div class="form-control w-full">
				<label class="label">
					<span class="label-text text-base-content">用户名/学号</span>
				</label>
				<input
					v-model="state.credentials.username"
					class="input input-bordered w-full"
					type="text"
				/>
			</div>

			<div class="form-control w-full">
				<label class="label">
					<span class="label-text text-base-content">密码</span>
				</label>
				<input
					v-model="state.credentials.password"
					class="input input-bordered w-full"
					type="password"
				/>
			</div>

			<fieldset class="grid grid-cols-2">
				<label class="label">
					<input
						v-model="state.credentials.rememberMe"
						class="checkbox"
						type="checkbox"
					/>
					记住我
				</label>
				<label class="label">
					<input
						v-model="state.credentials.autoLogin"
						class="checkbox"
						type="checkbox"
					/>
					自动登录
				</label>
			</fieldset>

			<button
				class="btn w-full"
				:class="{
					'btn-accent': !loading,
					'btn-disabled': loading,
				}"
				:disabled="loading"
				type="submit"
			>
				<span v-if="loading" class="loading loading-spinner" />
				登录
			</button>
		</form>
	</div>
</template>
