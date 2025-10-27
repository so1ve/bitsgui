import { reactive } from "vue";

import { getCredentials } from "./api";

const credentials = await getCredentials();

export const state = reactive({
	firstOpen: true,
	manualLogout: false,
	credentials,
});
