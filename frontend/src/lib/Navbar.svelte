<script lang="ts">
	import { z } from 'zod';
	import type { userSchema } from '$lib/schemas';
	import { backendUrl } from '$lib/data';
	export let user: z.infer<typeof userSchema> | null;
</script>

<nav class="py-1 px-2 text-white">
	{#if user != null}
		<!-- {user} -->
		<a href={`${backendUrl}/logout/discord`} class="underline underline-offset-2">sign out</a>
	{:else}
		<a href={`${backendUrl}/login/discord`} class="underline underline-offset-2">sign in</a>
	{/if}
</nav>
