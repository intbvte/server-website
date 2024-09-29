<script lang="ts">
	import { z } from 'zod';
	import type { userSchema } from '$lib/schemas';
	import { backendUrl } from '$lib/data';
	export let user: z.infer<typeof userSchema> | null;

	let action = user != null ? "logout" : "login";
	let text = user != null ? "Out" : "In";
</script>

<nav class="p-2.5 text-black">
	<a href={`${backendUrl}/${action}/discord`} class="p-0.5 mc-gray">Sign {text}</a>
</nav>
