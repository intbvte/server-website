<script lang="ts">
	import { z } from 'zod';
	import type { userSchema } from '$lib/schemas';
	import { backendUrl } from '$lib/data';
	export let user: z.infer<typeof userSchema> | null;
</script>

<nav class="p-2">
	{#if user != null}
		{user.discord_username}
	{:else}
		<a href={`${backendUrl}/login/discord`} target="_blank">log in</a>
	{/if}
</nav>
