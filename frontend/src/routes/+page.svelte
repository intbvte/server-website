<script lang="ts">
	import { backendUrl } from '$lib/data';
	import { minecraftUserDataSchema } from '$lib/schemas';
	import Skin from '$lib/Skin.svelte';
	import { day, hour, minute, second } from '$lib/time';
	import Whitelist from '$lib/Whitelist.svelte';
	import type { PageData } from './$types';

	import { dev } from '$app/environment';
	// const dev = false
	import WhitelistModal from '$lib/WhitelistModal.svelte';

	export let data: PageData;

	let whitelistModal:WhitelistModal;


	const releaseDate = new Date(1728154800000)
	let remainingTime = releaseDate.valueOf() - Date.now().valueOf();
	const getRemainingTimeString = () => {
		let rts = remainingTime;
		
		const hours = Math.floor(rts / hour);
		rts %= hour;
		
		const minutes = Math.floor(rts / minute);
		rts %= minute;
		
		const seconds = Math.floor(rts / second);
		
		return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2,"0")}:${seconds.toString().padStart(2,"0")}`
	}
	let remainingTimeString = getRemainingTimeString();
	setInterval(()=>{
		remainingTime = releaseDate.valueOf() - Date.now().valueOf();

		remainingTimeString = getRemainingTimeString();
	},1000)

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
			{#if remainingTime > 0}
				<div
					class="col-span-2 mc-info p-4 text-center border-12 text-black flex flex-col pixelated"
				>
					{#if remainingTime < day}
						release in {remainingTimeString}

					{:else}
						Server start on <br> {releaseDate.toLocaleDateString()}
						at {releaseDate.toLocaleTimeString()}
					{/if}
				</div>
			{/if}
			{#if data.user && !data.user.minecraft_uuid && (remainingTime < 0 || data.user.is_admin || dev)}
				<Whitelist/>
			{/if}
		</div>
		{#if dev || data.user && data.user.is_admin} <!--FIXME/TODO not finished yet, restricted to admins only -->
			<a href="/docs/rules" class="mc-button text-white p-2 text-center pixelated"> Rules </a>
			<a href="/docs/faq" class="mc-button text-white p-2 text-center pixelated"> FAQ </a>
		{/if}
		<div class="flex justify-between py-1 sm:col-span-2 mc-dark divide-x-2 divide-[#202020] mx-auto">
			{#if remainingTime < 0 || dev}
				<a href="https://ctm.railways.ithundxr.dev/" class="w-full"> <img src="/ui/trackmap_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
				<a href="https://map.railways.ithundxr.dev/" class="w-full"> <img src="/ui/bluemap_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
			{/if}
			{#if remainingTime < day || dev}
				<a href="https://modrinth.com/modpack/steam-n-rails-modpack" class="w-full"> <img src="/ui/modrinth_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
				{/if}
			<a href="https://opencollective.com/railways" class="w-full"> <img src="/ui/opencollective_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
			<a href="https://discord.gg/create-steam-n-rails-706277846389227612" class="w-full"> <img src="/ui/discord_logo.png" width="48" class="pixelated mx-auto px-2" alt=""></a>
		</div>
		{#if data.user && data.user.minecraft_uuid}
			<div class="absolute right-full w-32 flex flex-col items-center gap-2 m-2">
				<div class="drop-shadow-md w-full h-48">
					<Skin data={{uuid: data.user.minecraft_uuid}}/>
				</div>
				<span class="bg-black/50 text-white px-2 py-0.5">{minecraftUsername}</span>
				<input type="button" class="text-white text-xs hover:underline cursor-pointer" on:click={whitelistModal.openModal} value="edit"/>
			</div>
			<WhitelistModal usernameInput={minecraftUsername} bind:this={whitelistModal}/>
		{/if}
	</div>
</main>

