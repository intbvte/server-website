<script lang="ts">
	import '../app.css';
	import type { PageData } from './$types';
	import Navbar from '$lib/Navbar.svelte';

	export let data: PageData;
</script>

<div class="bg-sky w-full">
	<div class="absolute bottom-0 w-screen overflow-x-clip">
		<!-- <img
			src="background/background_back.png"
			class="w-full absolute bottom-0 pixelated"
			alt="Background Back"
		/>
		<img
			src="background/background_train.png"
			class="w-full absolute bottom-0 pixelated train-move"
			alt="Background Back"
		/>
		<img
			src="background/background_front.png"
			class="w-full absolute bottom-0 pixelated"
			alt="Background Back"
		/> -->
	</div>
	<div class="w-screen h-full absolute z-10 left-0 top-0">
		<Navbar user={data.user} />
		<slot />
		<p></p>
	</div>
</div>
