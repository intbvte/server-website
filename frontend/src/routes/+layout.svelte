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

<div class="w-full bg-cover min-h-screen bg-center pixelated">
	<div class="absolute bottom-0 w-screen overflow-x-clip h-full">
		<Panorama/>
	</div>
	<div class="w-screen h-full z-10 absolute left-0 top-0">
		<Navbar user={data.user} />
		<slot />
		<p></p>
	</div>
</div>
