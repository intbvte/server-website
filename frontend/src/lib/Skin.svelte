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

    export let username:string;

    let gltf:GLTF|null;
    const imageLoader = new THREE.TextureLoader();


    const loadSkin = async (username:string) => {
        if(!gltf) return;
        console.log(username)
        const uuidData = await fetch(`${backendUrl}/users/username_to_uuid/minecraft/${username}`)
        .catch(reason=>{
            alert("profile is incorrect")
            throw "";
        })
        .then(e=>e.json())

        const {id: uuid} = uuidSchema.parse(uuidData);

        console.log(uuidData)

        const userData = await fetch(`${backendUrl}/users/id_to_username/${uuid}`).then(e=>e.json())

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
        const camera = new THREE.PerspectiveCamera(75, element.clientWidth / element.clientHeight, 0.1, 1);
        const renderer = new THREE.WebGLRenderer();
        renderer.outputColorSpace = THREE.LinearDisplayP3ColorSpace;
        renderer.setSize(element.clientWidth, element.clientHeight);
        element.appendChild(renderer.domElement)
    
        // Create cube geometry for the panorama
        const modelLoader = new GLTFLoader()

        gltf = await modelLoader.loadAsync("/player.glb")

        scene.add(gltf.scene)
    
        // Set camera position inside the cube
        camera.position.set(-5, 0, 0);
        // camera.rotation.x = -.1

    
        // // Animate the scene
        function animate() {
            requestAnimationFrame(animate);
            gltf!.scene.rotation.y += 0.0005;
            renderer.render(scene, camera);
        }

        window.addEventListener('resize', () => {
            camera.aspect = element.clientWidth / element.clientHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(element.clientWidth, element.clientHeight);
        });
    
        animate();
        loadSkin(username)
    })

    $: username, loadSkin(username);
</script>

<div bind:this={element} class="w-full h-full m-0"/>