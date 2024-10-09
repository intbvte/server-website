<script lang="ts">
	import { backendUrl } from '$lib/data';
	import { minecraftUserDataSchema } from '$lib/schemas';
	import Skin from '$lib/Skin.svelte';
	import Whitelist from '$lib/Whitelist.svelte';
	import type { PageData } from './$types';

	import WhitelistModal from '$lib/WhitelistModal.svelte';

	export let data: PageData;

	let whitelistModal:WhitelistModal;

	let minecraftUsername:string = "";
	if(data.user && data.user.minecraft_uuid)
		fetch(`${backendUrl}/users/id_to_username/minecraft/${data.user?.minecraft_uuid}`)
			.then(res=>res.json())
			.then(minecraftUserDataSchema.parseAsync)
			.then(user=>minecraftUsername = user.minecraft_username)
</script>

<main class="max-w-screen-lg w-full mx-auto flex items-center flex-col my-10 gap-10 relative">
	<img src="title.png" alt="Steam 'n' Rails SMP Season 2" class="max-w-2xl w-[95%] px-2" />
	<div class="grid grid-cols sm:grid-cols-2 gap-3 w-full max-w-sm drop-shadow-xl shadow-black">
		<div class="sm:col-span-2 flex flex-col gap-2">
			{#if data.user && !data.user.minecraft_uuid}
				<Whitelist/>
			{/if}
		</div>
			<a href="/docs/rules" class="mc-button text-white p-2 text-center pixelated"> Rules </a>
			<a href="/docs/faq" class="mc-button text-white p-2 text-center pixelated"> FAQ </a>
			<a href="/mods" class="mc-button text-white p-2 text-center pixelated"> Modlist </a>
			<a href="/docs/guilds" class="mc-button text-white p-2 text-center pixelated"> Economy </a>
		<div class="flex justify-between py-1 sm:col-span-2 mc-dark divide-x-2 divide-[#202020] mx-auto">
			<!-- <a href="https://ctm.railways.ithundxr.dev/" target="_blank" class="w-full"> <img src="/ui/trackmap_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a> -->
			<a href="https://map.railways.ithundxr.dev/" target="_blank" class="w-full"> <img src="/ui/bluemap_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
			<a href="https://modrinth.com/modpack/steam-n-rails-modpack" target="_blank" class="w-full"> <img src="/ui/modrinth_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
			<a href="https://opencollective.com/railways" target="_blank" class="w-full"> <img src="/ui/opencollective_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
			<a href="https://discord.gg/create-steam-n-rails-706277846389227612" target="_blank" class="w-full"> <img src="/ui/discord_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
		</div>
		{#if data.user && data.user.minecraft_uuid}
			<div class="sm:absolute mx-auto sm:mx-5 sm:col-span-2 right-full w-32 flex flex-col items-center m-2">
				<span class="bg-black/50 text-white px-2 py-0.5">{minecraftUsername}</span>
				<div class="drop-shadow-md w-full h-48">
					<Skin data={{uuid: data.user.minecraft_uuid}}/>
				</div>
				<input type="button" class="text-white/50 text-xs hover:underline cursor-pointer" on:click={whitelistModal.openModal} value="Change"/>
			</div>
			<WhitelistModal usernameInput={minecraftUsername} bind:this={whitelistModal}/>
		{/if}
	</div>
</main>

