<script lang="ts">
	import type { PageData } from '../../.svelte-kit/types/src/routes/$types';
	import { backendUrl } from '$lib/data';

	export let data: PageData;
</script>

<main class="max-w-screen-lg w-full mx-auto flex items-center flex-col my-10 gap-10">
	<img src="logo.png" alt="Steam 'n' Rails SMP Season 2" class="max-w-2xl" />
	<div class="grid grid-cols-2 gap-3 w-full max-w-sm">
		<div class="col-span-2">
			{#if data.user && !data.user.minecraft_uuid}
				<form
					class="bg-lightgray flex flex-col items-center"
					action={`${backendUrl}/minecraft/username/change`}
					method="post"
				>
					<h3 class="w-full text-center bg-gold pixelated border-8 text-lg text-darkred">
						Get Whitelisted
					</h3>
					<div class="p-3 flex bg-gray pixelated border-4 w-full gap-1">
						<input
							type="text"
							class="bg-input pixelated px-1 text-white outline-none border-8 placeholder-white w-full text-shadow shadow-[#3e3e3e]"
							name="username"
							placeholder="Minecraft username"
							autocomplete="off"
						/>
						<button type="submit">
							<img src="ui/confirm.png" class="pixelated" width="40" alt="confirm" />
						</button>
					</div>
				</form>
			{:else if data.user}
				<div
					class="col-span-2 bg-info p-4 text-center border-12 text-black flex flex-col pixelated"
				>
					You are whitelisted
				</div>
			{/if}
		</div>
		<a href="/rules" class="bg-gray p-2 border-4 text-center pixelated"> Rules </a>
		<a href="/guilds" class="bg-gray p-2 border-4 text-center pixelated"> Guilds </a>
		<a href="https://ctm.railways.ithundxr.dev/" class="bg-gray p-2 border-4 text-center pixelated"> Track Map </a>
		<a href="https://map.railways.ithundxr.dev/" class="bg-gray p-2 border-4 text-center pixelated"> BlueMap </a>
	</div>
</main>
