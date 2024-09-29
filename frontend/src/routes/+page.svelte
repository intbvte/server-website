<script lang="ts">
	import { backendUrl } from '$lib/data';
	import { minecraftUserDataSchema } from '$lib/schemas';
	import Skin from '$lib/Skin.svelte';
	import { day, hour, minute, second } from '$lib/time';
	import Whitelist from '$lib/Whitelist.svelte';
	import type { PageData } from './$types';

	export let data: PageData;


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
		<div class="sm:col-span-2">
			{#if data.user && !data.user.minecraft_uuid && remainingTime < 0}
				<Whitelist/>
			{:else}
				<div
					class="col-span-2 bg-info p-4 text-center border-12 text-black flex flex-col pixelated bg-input"
				>
					{#if remainingTime < 0}
						{#if data.user}
							You are whitelisted<br>
						{:else}
							Sign in to get whitelisted
						{/if}
					{:else if remainingTime < day}
						release in {remainingTimeString}

					{:else}
						Server start on <br> {releaseDate.toLocaleDateString()}
						at {releaseDate.toLocaleTimeString()}
					{/if}
				</div>
			{/if}
		</div>
		<!-- FIXME/TODO commented out because it's not ready yet -->
		<a href="/rules" class="bg-button text-white p-2 text-center pixelated"> Rules </a>
		<a href="/faq" class="bg-button text-white p-2 text-center pixelated"> FAQ </a>
		
		<!-- <a href="https://modrinth.com/modpack/steam-n-rails-modpack" class="bg-modrinth text-white p-2 text-center pixelated"> Modrinth </a>
		<a href="https://opencollective.com/railways" class="bg-opencollective text-[#1041a3] p-2 text-center pixelated"> Donate </a> -->
		<div class="flex justify-between py-1 sm:col-span-2 bg-dark divide-x-2 divide-[#202020]">
			{#if remainingTime < 0}
			<!-- <a href="https://ctm.railways.ithundxr.dev/" class="bg-button text-white p-2 text-center pixelated"> Track Map </a>
			<a href="https://map.railways.ithundxr.dev/" class="bg-bluemap text-white p-2 text-center pixelated"> BlueMap </a> -->
			<a href="https://ctm.railways.ithundxr.dev/" class="w-full"> <img src="/ui/trackmap_logo.png" width="32" class="pixelated mx-auto" alt=""></a>
			<a href="https://map.railways.ithundxr.dev/" class="w-full"> <img src="/ui/bluemap_logo.png" width="32" class="pixelated mx-auto" alt=""></a>
			{/if}
			{#if remainingTime < day}
				<a href="https://modrinth.com/modpack/steam-n-rails-modpack" class="w-full"> <img src="/ui/modrinth_logo.png" width="32" class="pixelated mx-auto" alt=""></a>
				{/if}
			<a href="https://opencollective.com/railways" class="w-full"> <img src="/ui/opencollective_logo.png" width="32" class="pixelated mx-auto" alt=""></a>
			<a href="https://discord.gg/create-steam-n-rails-706277846389227612" class="w-full"> <img src="/ui/discord_logo.png" width="32" class="pixelated mx-auto" alt=""></a>
		</div>
		{#if data.user && data.user.minecraft_uuid}
			<div class="absolute right-full w-32 flex flex-col items-center gap-2 m-2">
				<div class="drop-shadow-md w-full h-48">
					<Skin data={{uuid: data.user.minecraft_uuid}}/>
				</div>
				<span class="bg-black/50 text-white px-2 py-0.5">{minecraftUsername}</span>
			</div>
		{/if}
	</div>
</main>

