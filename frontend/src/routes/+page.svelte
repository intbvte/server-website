<script lang="ts">
	import { backendUrl } from '$lib/data';
	import { minecraftUserDataSchema } from '$lib/schemas';
	import Skin from '$lib/Skin.svelte';
	import Whitelist from '$lib/Whitelist.svelte';
	import type { PageData } from './$types';

	export let data: PageData;


	const releaseDate = new Date(172815480000.0)
	let remainingTime = releaseDate.valueOf() - Date.now().valueOf();
	const getRemainingTimeString = () => {
		let rts = remainingTime;
		
		const hours = Math.floor(rts / (1000 * 60 * 60));
		rts %= 1000 * 60 * 60;
		
		const minutes = Math.floor(rts / (1000 * 60));
		rts %= 1000 * 60;
		
		const seconds = Math.floor(rts / 1000);
		
		return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2,"0")}:${seconds.toString().padStart(2,"0")}`
	}
	let remainingTimeString = getRemainingTimeString();
	setInterval(()=>{
		remainingTime = releaseDate.valueOf() - Date.now().valueOf();

		remainingTimeString = getRemainingTimeString();
	},1000)

	let minecraftUsername:string = "";
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
					{:else if remainingTime < (1000 * 60 * 60 * 24 * 1)}
						release in {remainingTimeString}

					{:else}
						Server start on <br> {releaseDate.toLocaleDateString()}
						at {releaseDate.toLocaleTimeString()}
					{/if}
				</div>
			{/if}
		</div>
		<a href="/rules" class="bg-button text-white p-2 text-center pixelated"> Rules </a>
		<a href="/guilds" class="bg-button text-white p-2 text-center pixelated"> Guilds </a>
		<a href="https://ctm.railways.ithundxr.dev/" class="bg-button text-white p-2 text-center pixelated"> Track Map </a>
		<a href="https://map.railways.ithundxr.dev/" class="bg-button text-white p-2 text-center pixelated"> BlueMap </a>
		{#if data.user && data.user.minecraft_uuid}
			<div class="absolute right-full w-32 flex flex-col items-center gap-2 m-2">
				<div class="drop-shadow-md w-full h-32">
					<Skin data={{uuid: data.user.minecraft_uuid}}/>
				</div>
				<span class="bg-black/50 text-white px-2 py-0.5">{minecraftUsername}</span>
			</div>
		{/if}
	</div>
</main>

