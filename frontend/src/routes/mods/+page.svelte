<script lang="ts">
	import Project from '$lib/Project.svelte';
    import type { PageData } from './$types';
    export let data:PageData;
    const getProjectData = (project: typeof data.latest.dependencies[number]) =>{
        if(project.project_id != null) return data.projectMap.get(project.project_id)!
        return project
    }
</script>

<main class="max-w-3xl w-full mx-auto flex items-start flex-col my-10 gap-2 relative pb-32">
    <a href=".." class="mc-gold px-2">Back</a>
    <a href="https://modrinth.com/modpack/steam-n-rails-modpack" target="_blank">
        <h1 class=" text-2xl px-2 mc-gray">Modlist v{data.latest.version_number}</h1>
    </a>
    <div class=" w-full flex flex-col gap-2">
        {#each data.latest.dependencies.sort(dep=>dep.project_id ? -1 : 1) as dependency}
            <Project project={getProjectData(dependency)}/>
        {/each}
    </div>
</main>