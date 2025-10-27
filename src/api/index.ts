import { invoke } from "@tauri-apps/api/core";
import { useThrottleFn } from "@vueuse/core";

import type { ApiResponse, Credentials, SrunLoginState } from "../types";

export async function initBitsrun() {
	await invoke("init");
}

export const checkStatus = useThrottleFn(
	async () => await invoke<ApiResponse<SrunLoginState, string>>("check_status"),
	1000,
);

export async function isLoggedIn() {
	const response = await checkStatus();

	return response.success && response.data.error === "ok";
}

export const login = async (credentials: Credentials) =>
	await invoke<ApiResponse<SrunLoginState, string>>(
		"login",
		credentials as any,
	);

export const logout = async (username: string) =>
	await invoke<ApiResponse<string, string>>("logout", {
		username,
	});

export { getCredentials, initStore, setCredentials } from "./store";
