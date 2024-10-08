<script lang="ts">
	import { backendUrl } from '$lib/data';
	import Skin from './Skin.svelte';
    
	let username:string;
	export let usernameInput:string;

	let dialog:HTMLDialogElement;

    export function openModal() {
        if(!usernameInput) return
		username = usernameInput
		if(!dialog.open) dialog.showModal();
    }

	const submitToWhitelist = async () => {
		const res = await fetch(`${backendUrl}/minecraft/username/change`, {
			method: "POST",
			credentials: "include",
			headers: {
				"Content-Type": "application/x-www-form-urlencoded",
			},
			body: `username=${username}`,
			mode: "no-cors"
		})
		if(!res.ok)
			alert(res.statusText)
		window.location.reload();
	}
</script>

<dialog bind:this={dialog} class="max-w-sm w-full backdrop:bg-dither backdrop:opacity-90">
	<!-- <form method="dialog"> -->
		<h3 class="w-full text-center mc-gold pixelatedtext-lg text-darkred relative">
			Get Whitelisted
            <input type="button" class="absolute right-2 cursor-pointer" value="x" on:click={()=>dialog.close()}>
		</h3>
		<div class="p-3 flex flex-col mc-gray pixelated w-full gap-1">
			<div class="flex flex-row gap-1">
				<input
					type="text"
					class="mc-input pixelated px-1 text-white outline-none placeholder-white w-full text-shadow block placeholder:text-lightgray disabled:text-lightgray"
					name="username"
					placeholder="Minecraft username"
					autocomplete="off"
					spellcheck="false"
					maxlength="16"
					bind:value={usernameInput}
					on:focusout={()=>username = usernameInput}
					on:keyup={e=>{if(e.key == "Enter") username = usernameInput}}
				/>
			</div>
			{#if username}
				<div class="p-4 mc-dark h-96">
					<Skin data={{username}}/>
				</div>
				<input type="button" class="mc-button text-white p-2 text-center pixelated cursor-pointer" value="Add to whitelist" on:click={submitToWhitelist}>
			{/if}
		</div>
	<!-- </form> -->
</dialog>
