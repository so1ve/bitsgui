<script setup lang="ts">
import { Toaster } from "vue-sonner";

import "vue-sonner/style.css";
</script>

<template>
	<Toaster position="top-center" rich-colors />
	<div class="min-h-dvh min-w-dvw p-3">
		<RouterView v-slot="{ Component }">
			<template v-if="Component">
				<Transition mode="out-in">
					<Suspense>
						<component :is="Component" />

						<template #fallback>正在加载...</template>
					</Suspense>
				</Transition>
			</template>
		</RouterView>
	</div>
</template>
