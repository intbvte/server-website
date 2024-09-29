<script lang="ts">
	import { z } from 'zod';
	import type { userSchema } from '$lib/schemas';
	import { backendUrl } from '$lib/data';
	export let user: z.infer<typeof userSchema> | null;
</script>

<nav class="p-2.5 text-black">
	{#if user != null}
		<!-- {user} -->
		<a href={`${backendUrl}/logout/discord`} class="underline underline-offset-2 p-0.5 bg-gray">Sign Out</a>
	{:else}
		<a href={`${backendUrl}/login/discord`} class="underline underline-offset-2 p-0.5 bg-gray">Sign In</a>
	{/if}
</nav>
