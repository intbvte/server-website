<script lang="ts">
	import '../app.css';
	import type { PageData } from './$types';
	import Navbar from '$lib/Navbar.svelte';
	import Panorama from '$lib/Panorama.svelte';

	export let data: PageData;
</script>

<svelte:head>
	<title>Steam 'n' Rails SMP S2</title>
</svelte:head>

<div class="w-full bg-cover min-h-svh bg-center pixelated overflow-hidden font-minecraft">
	<div class="fixed bottom-0 w-full overflow-hidden h-full brightness-50">
		<Panorama/>
	</div>
	<div class="w-full h-full z-10 absolute left-0 top-0">
		<slot />
		<div class="fixed bottom-0">
			<Navbar user={data.user} />
		</div>
	</div>
</div>
