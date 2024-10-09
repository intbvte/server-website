import { projectsSchema, versionsSchema } from '$lib/schemas.js';

export async function load({ fetch }){
    const modrinthAPIBaseUrl = "https://api.modrinth.com/v2"
    const versions = versionsSchema.parse(await (await fetch(modrinthAPIBaseUrl+"/project/9a0qj1oH/version", {cache: "force-cache"})).json())
    const latest = versions[0]
    const projectIDs = latest.dependencies
        .filter(dependency=>dependency.project_id != null)
        .map(dependency=>dependency.project_id);
    const projects = projectsSchema.parse(await (await fetch(`${modrinthAPIBaseUrl}/projects?ids=${encodeURIComponent(`[${projectIDs.map(p=>`"${p}"`).join()}]`)}`, {cache: "force-cache"})).json());
    const projectMap = new Map(projects.map(project=>[project.id, project]))
    return {latest, projectMap}
}