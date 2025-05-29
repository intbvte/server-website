<script lang="ts">
	import type { z } from 'zod';
	import WhitelistModal from './WhitelistModal.svelte';
	import type { userSchema } from './schemas';
	let usernameInput: string;
	let whitelistModal: WhitelistModal;
	export let user: z.infer<typeof userSchema> | null;
</script>

<div class="flex flex-col items-center">
	{#if user}
		<h3 class="w-full text-center mc-gold pixelated text-lg text-darkred">Get Whitelisted</h3>
		<div class="p-3 flex mc-gray pixelated w-full gap-1">
			<input
				type="text"
				class="mc-input pixelated px-1 text-white outline-none border-8 placeholder-white w-full block placeholder:text-lightgray"
				name="username"
				placeholder="Minecraft username"
				autocomplete="off"
				spellcheck="false"
				bind:value={usernameInput}
				maxlength="16"
				on:keyup={(e) => {
					if (e.key == 'Enter') whitelistModal.openModal();
				}}
			/>
			<button type="button" on:click={whitelistModal.openModal} class="outline-none">
				<img src="ui/next.png" class="pixelated" width="48px" alt="confirm" />
			</button>
		</div>
	{:else}
		<a href="/backend/login/discord" class="w-full">
			<h3 class="w-full text-center mc-gray pixelated text-lg text-black">
				Sign In to Get Whitelisted
			</h3>
		</a>
	{/if}
</div>

<WhitelistModal bind:usernameInput bind:this={whitelistModal} />
