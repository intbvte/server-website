<script lang="ts">
	import { z } from 'zod';
	import type { userSchema } from '$lib/schemas';
	import { backendUrl } from '$lib/data';
	export let user: z.infer<typeof userSchema> | null;

	const signIn = () => {
		open(`${backendUrl}/login/discord`, "signIn", "width=640,height=640")?.addEventListener("close", location.reload)
	}
</script>

<nav class="py-1 px-2 text-white">
	{#if user != null}
		<!-- {user} -->
		<a href={`${backendUrl}/logout/discord`} target="signOut" class="underline underline-offset-2">sign out</a>
	{:else}
		<input type="button" value="sign in" on:click={signIn}>
		<!-- <a href={`${backendUrl}/login/discord`} target="signIn">sign in</a> -->
	{/if}
</nav>
