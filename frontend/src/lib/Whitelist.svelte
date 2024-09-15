<script lang="ts">
	import { backendUrl } from '$lib/data';
    
	let username:string;
	let usernameInput:string;

	const openModal = () => {
		if(!usernameInput) return
		username = usernameInput
		if(!dialog.open) dialog.showModal();
	}
	let dialog:HTMLDialogElement;

	const submitToWhitelist = async () => {
		await fetch(`${backendUrl}/minecraft/username/change`, {
			method: "POST",
			credentials: "include",
			headers: {
				"Content-Type": "application/x-www-form-urlencoded",
			},
			body: `username=${username}`,
			mode: "no-cors"
		})
		window.location.reload();
	}
</script>

<div
class="flex flex-col items-center"
>
<h3 class="w-full text-center bg-gold pixelated text-lg text-darkred">
    Get Whitelisted
</h3>
<div class="p-3 flex bg-gray pixelated w-full gap-1">
    <input
        type="text"
        class="bg-input pixelated px-1 text-white outline-none border-8 placeholder-white w-full block placeholder:text-lightgray"
        name="username"
        placeholder="Minecraft username"
        autocomplete="off"
        spellcheck="false"
        bind:value={usernameInput}
        on:keyup={e=>{if(e.key == "Enter") openModal()}}
    />
    <button type="button" on:click={openModal} class="outline-none">
        <img src="ui/next.png" class="pixelated" width="48px" alt="confirm" />
    </button>
</div>
</div>

<dialog bind:this={dialog} class="max-w-sm w-full backdrop:bg-dither backdrop:opacity-90">
	<!-- <form method="dialog"> -->
		<h3 class="w-full text-center bg-gold pixelatedtext-lg text-darkred">
			Get Whitelisted
		</h3>
		<div class="p-3 flex flex-col bg-gray pixelated w-full gap-1">
			<div class="flex flex-row gap-1">
				<input
					type="text"
					class="bg-input pixelated px-1 text-white outline-none placeholder-white w-full text-shadow block placeholder:text-lightgray disabled:text-lightgray"
					name="username"
					placeholder="Minecraft username"
					autocomplete="off"
					spellcheck="false"
					bind:value={usernameInput}
					on:focusout={()=>username = usernameInput}
					on:keyup={e=>{if(e.key == "Enter") username = usernameInput}}
					autofocus
				/>
			</div>
			<img src={`https://vzge.me/frontfull/832/${username}.png`} alt={`player skin of ${username}`} class="bg-dark p-4">
			<input type="button" class="bg-button text-white p-2 text-center pixelated cursor-pointer" value="Add to whitelist" on:click={submitToWhitelist}>
		</div>
	<!-- </form> -->
</dialog>
