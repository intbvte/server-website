<script lang="ts">
	import type { PageData } from './$types';
	import { backendUrl } from '$lib/data';

	export let data: PageData;

	let username:string;
	let dialog:HTMLDialogElement;
</script>

<main class="max-w-screen-lg w-full mx-auto flex items-center flex-col my-10 gap-10">
	<img src="title.png" alt="Steam 'n' Rails SMP Season 2" class="max-w-2xl" />
	<div class="grid grid-cols-2 gap-3 w-full max-w-sm">
		<div class="col-span-2">
			{#if data.user && !data.user.minecraft_uuid}
				<form
					class="flex flex-col items-center"
				>
					<h3 class="w-full text-center bg-gold pixelated border-8 text-lg text-darkred">
						Get Whitelisted
					</h3>
					<div class="p-3 flex bg-gray pixelated border-4 w-full gap-1">
						<input
							type="text"
							class="bg-input pixelated px-1 text-white outline-none border-8 placeholder-white w-full text-shadow block placeholder:text-lightgray"
							name="username"
							placeholder="Minecraft username"
							autocomplete="off"
							bind:value={username}
						/>
						<button type="button" on:click={()=>dialog.showModal()}>
							<img src="ui/next.png" class="pixelated" width="48px"  alt="confirm" />
						</button>
					</div>
				</form>
			{:else if data.user}
				<div
					class="col-span-2 bg-info p-4 text-center border-12 text-black flex flex-col pixelated bg-input"
				>
					You are whitelisted
				</div>
			{/if}
		</div>
		<a href="/rules" class="bg-gray p-2 border-4 text-center pixelated"> Rules </a>
		<a href="/rules" class="bg-gray p-2 border-4 text-center pixelated"> Guilds </a>
		<a href="/rules" class="bg-gray p-2 border-4 text-center pixelated"> Track Map </a>
		<a href="/rules" class="bg-gray p-2 border-4 text-center pixelated"> BlueMap </a>
	</div>
</main>

<dialog bind:this={dialog} class="max-w-sm w-full">
	<form method="dialog">
		<h3 class="w-full text-center bg-gold pixelated border-8 text-lg text-darkred">
			Get Whitelisted
		</h3>
		<div class="p-3 flex bg-gray pixelated border-4 w-full gap-1">
			<input
				type="text"
				class="bg-input pixelated px-1 text-white outline-none border-8 placeholder-white w-full text-shadow block placeholder:text-lightgray disabled:text-lightgray"
				name="username"
				placeholder="Minecraft username"
				autocomplete="off"
				bind:value={username}
			/>
			<button type="button" on:click={()=>dialog.showModal()}>
				<img src="ui/next.png" class="pixelated" width="48px"  alt="confirm" />
			</button>
		</div>
	</form>
</dialog>
