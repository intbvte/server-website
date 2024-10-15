<script lang="ts">
  import { fetchWithSchema } from "$lib";
	import type { z } from "zod";
	import { serverStatusSchema } from "./schemas";
	import { onMount } from "svelte";

  export let ip:string;

  let data:z.infer<typeof serverStatusSchema>|undefined;
  onMount(async()=>{
    const status = await fetchWithSchema(new Request(`https://api.mcsrvstat.us/3/${ip}`), serverStatusSchema)
    if(!status.success) return;
    data = status.data;
  })
</script>

<div class="mc-dark text-white text-center">
  {#if data}
    <div class="inline">Server <span class:text-green-500={data.online} class:text-red-500={!data.online}>{data.online ? "Online" : "Offline"}</span></div>
    {#if data.online}
      <div class="inline">({data.players.online}/{data.players.max})</div>
    {/if}
    {:else}
      Server status loading
    {/if}
    <div class="text-sm text-gray">{data ? data.hostname : ip}</div>
</div>

