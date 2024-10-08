<script lang="ts">
	import WhitelistModal from "./WhitelistModal.svelte";
	let usernameInput:string;
	let whitelistModal:WhitelistModal;
</script>

<div
class="flex flex-col items-center"
>
<h3 class="w-full text-center mc-gold pixelated text-lg text-darkred">
    Get Whitelisted
</h3>
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
        on:keyup={e=>{if(e.key == "Enter") whitelistModal.openModal()}}
    />
    <button type="button" on:click={whitelistModal.openModal} class="outline-none">
        <img src="ui/next.png" class="pixelated" width="48px" alt="confirm" />
    </button>
</div>
</div>

<WhitelistModal bind:usernameInput={usernameInput} bind:this={whitelistModal}/>
