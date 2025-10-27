<script setup lang="ts">
import { useOverlayScrollbars } from "overlayscrollbars-vue";

import "overlayscrollbars/overlayscrollbars.css";

const [initBodyOverlayScrollbars] = useOverlayScrollbars({
	defer: true,
	options: {
		scrollbars: {
			clickScroll: true,
		},
	},
});

initBodyOverlayScrollbars({
	target: document.body,
	cancel: {
		body: false,
	},
});
</script>
