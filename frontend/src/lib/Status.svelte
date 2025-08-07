<script lang="ts">
	import { safeFetchWithSchema } from '$lib';
	import type { z } from 'zod';
	import { serverStatusSchema } from './schemas';
	import { onMount } from 'svelte';

	export let ip: string;

	let data: z.infer<typeof serverStatusSchema> | undefined;
	let isSleeping: boolean;
	onMount(async () => {
		const status = await safeFetchWithSchema(
			new Request(`https://api.mcsrvstat.us/3/${ip}`),
			serverStatusSchema
		);
		if (!status.success) return;
		data = status.data;

		// @ts-expect-error: Works for now.
		isSleeping = status.data.protocol.version == -1
	});
</script>

{#if data && isSleeping}
	<div class="mc-dark text-white text-center text-sm">
		<span class="m-4">Server is asleep! <b>Join to wake it up!</b></span>
	</div>
{/if}

<div class:text-center={isSleeping} class:text-left={!isSleeping} class="flex items-center mc-dark text-white">
	{#if data}
		{#if !isSleeping}
			<img class="h-12 w-12 mr-4 m-2 rounded-md" src={data.icon} alt="">
		{/if}
		<div class="flex flex-col w-full">
			<div class="inline">
				Server <span class:text-green-500={!isSleeping} class:text-blue-500={isSleeping}
					>{!isSleeping ? 'Online' : 'Sleeping'}</span
				>
				{#if data.online && !isSleeping}
					<div class="inline">({data.players.online}/{data.players.max})</div>
				{/if}
			</div>
			<div class="text-sm text-gray">{ip} {#if !isSleeping}
				◉ v{data.protocol.version}
			{/if}</div>
		</div>
	{:else}
		Server status loading
	{/if}
</div>