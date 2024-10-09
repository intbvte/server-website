<script lang="ts">
	import type { z } from "zod";
	import type { projectsSchema } from "./schemas";

    export let project:z.infer<typeof projectsSchema>[number] | {
        project_id: null;
        version_id: null;
        file_name: string;
        dependency_type: string;
    }

    const getLuma = (rgb:number)=>{
        let r = (rgb >> 16) & 0xff;  // extract red
        let g = (rgb >>  8) & 0xff;  // extract green
        let b = (rgb >>  0) & 0xff;  // extract blue

        return 0.2126 * r + 0.7152 * g + 0.0722 * b; // per ITU-R BT.709
    }
    const dark = "color" in project && getLuma(project.color ?? 0xffffff) < 150;
</script>

<a class="flex mc-gray gap-2 p-2 items-center" href={"slug" in project ? `https://modrinth.com/${project.project_type}/${project.slug}` : "#"} target="_blank" style:background={"color" in project ? "#"+project.color?.toString(16).padStart(6, '0') : ""}>
    <img src={"icon_url" in project ? project.icon_url : ""} alt="" class="aspect-square w-12 pixelated mc-button">
    <div class="flex flex-col text-black" class:text-white={dark}>
        <h2>{"title" in project ? project.title : project.file_name}</h2>
        {#if "description" in project}
            <p class="text-xs text-black text-opacity-80" class:text-white={dark}>{project.description}</p>
        {/if}
    </div>
</a>
