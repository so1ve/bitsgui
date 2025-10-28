<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { openUrl } from "@tauri-apps/plugin-opener";

const version = await getVersion();

const repository = "https://github.com/so1ve/bitsgui";

async function openGithub() {
	await openUrl(repository);
}
</script>

<template>
	<footer
		class="footer footer-center p-2 mt-2 border-t border-dashed border-gray-400"
	>
		<a
			class="link link-hover"
			:href="repository"
			target="_blank"
			@click.prevent="openGithub"
		>
			BITSGUI v{{ version }}
		</a>
	</footer>
</template>
