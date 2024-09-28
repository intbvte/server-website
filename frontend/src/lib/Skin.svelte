<script lang="ts">
	import { onMount } from 'svelte';
    import * as THREE from 'three';
	import { backendUrl } from '$lib/data';
	import { minecraftProfileSchema, minecraftUserDataSchema, uuidSchema } from './schemas';
	import { GLTFLoader, type GLTF } from 'three/addons/loaders/GLTFLoader.js';
	import { Mesh } from 'three/src/objects/Mesh.js';
	import { MeshStandardMaterial } from 'three/src/materials/Materials.js';
	import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js';


    let element:HTMLDivElement;

    export let data:{username:string}|{uuid:string}


    let gltf:GLTF|null;
    const imageLoader = new THREE.TextureLoader();


    const loadSkin = async () => {
        if(!gltf) return;

        const uuid = "uuid" in data ? data.uuid : await (async ()=>{

            const uuidData = await fetch(`${backendUrl}/users/username_to_uuid/minecraft/${data.username}`)
            .catch(reason=>{
                alert("profile is incorrect")
                throw "";
            })
            .then(e=>e.json())
    
            const {id: uuid} = uuidSchema.parse(uuidData);
            return uuid;
        })()

        const userData = await fetch(`${backendUrl}/users/id_to_username/minecraft/${uuid}`).then(e=>e.json())

        const {properties} = minecraftUserDataSchema.parse(userData);

        const profile = minecraftProfileSchema.parse(JSON.parse(atob(properties[0].value)))

        const skinUrl = profile.textures.SKIN?.url ?? "";


        console.log(skinUrl)
        const texture = await imageLoader.loadAsync(skinUrl)

        gltf.scene.traverse(child=>{
            if(!(child instanceof Mesh)) return
            child.material = new MeshBasicMaterial({map: texture})
        })

        console.log(profile)
    }

    onMount(async ()=>{


        // Create scene, camera, and renderer
        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(30, element.clientWidth / element.clientHeight, 0.1, 100);
        const renderer = new THREE.WebGLRenderer({
            alpha: true
        });
        renderer.outputColorSpace = THREE.LinearDisplayP3ColorSpace;
        renderer.setSize(element.clientWidth, element.clientHeight);
        element.appendChild(renderer.domElement)
    
        // Create cube geometry for the panorama
        const modelLoader = new GLTFLoader()

        gltf = await modelLoader.loadAsync("/player.glb")

        scene.add(gltf.scene)
    
        // Set camera position inside the cube
        camera.position.set(0, 1, 4);
        // camera.rotation.x = -.1

    
        // // Animate the scene
        function animate() {
            requestAnimationFrame(animate);
            gltf!.scene.rotation.y += 0.005;
            renderer.render(scene, camera);
        }

        window.addEventListener('resize', () => {
            camera.aspect = element.clientWidth / element.clientHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(element.clientWidth, element.clientHeight);
        });
    
        renderer.render(scene, camera);

        animate();
        loadSkin()
    })

    $: data, loadSkin();
</script>

<div bind:this={element} class="w-full h-full min-h-24 m-0"/>

<!-- <img src={`https://vzge.me/frontfull/832/${ "username" in data ?  data.username : data.uuid }.png`} alt={`player skin of ${ "username" in data ?  data.username : data.uuid }`}> -->
