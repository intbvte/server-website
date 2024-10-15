<script lang="ts">
	import { onMount } from 'svelte';
    import * as THREE from 'three';
	import { backendUrl } from '$lib/data';
	import { minecraftProfileSchema, minecraftUserDataSchema, uuidSchema } from './schemas';
	import { GLTFLoader, type GLTF } from 'three/addons/loaders/GLTFLoader.js';
	import type { Mesh } from 'three/src/objects/Mesh.js';
	import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js';
	import type { Scene } from 'three/src/scenes/Scene.js';
	import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
	import { safeFetchWithSchema } from '$lib';


    let element:HTMLDivElement;

    export let data:{username:string}|{uuid:string}


    let gltf:GLTF|null;
    let scene:Scene|null;
    const imageLoader = new THREE.TextureLoader();


    const loadSkin = async () => {
        if(!window) return;
        console.log(gltf)

        const uuid = "uuid" in data ? data.uuid : await (async ()=>{
            const {success, data: uuidData} = await safeFetchWithSchema(new Request(`${backendUrl}/users/username_to_uuid/minecraft/${data.username}`), uuidSchema)
            if(!success) throw alert("profile is incorrect");
            const {id: uuid} = uuidData
            return uuid;
        })()

        const {success, data: userData} = await safeFetchWithSchema(new Request(`${backendUrl}/users/id_to_username/minecraft/${uuid}`), minecraftUserDataSchema)
        if(!success) throw alert("internal error");

        const {properties} = userData

        console.log(JSON.parse(atob(properties[0].value)))

        const profile = minecraftProfileSchema.parse(JSON.parse(atob(properties[0].value)))

        const skinUrl = profile.textures.SKIN?.url ?? "";

        let slim = profile.textures.SKIN?.metadata?.model == "slim";

        console.log({skinUrl, slim})
        const texture = await imageLoader.loadAsync(skinUrl)
        texture.minFilter = THREE.NearestFilter;
        texture.magFilter = THREE.NearestFilter;
        texture.flipY = false;

        
        console.log({scene})

        
        gltf = await modelLoader.loadAsync(slim ? "/slim.glb" : "/wide.glb")

        gltf.scene.position.set(0,-1.05,0)
        gltf.scene.rotateY(Math.PI)
        
        scene?.clear()
        scene?.add(gltf.scene)

        gltf.scene.traverse(child=>{
            if(!(child as Mesh).isMesh) return;
            (child as Mesh).material = new MeshBasicMaterial({map: texture, transparent: child.name.endsWith("Layer")});
        })

        console.log(profile)
    }

    const modelLoader = new GLTFLoader()

    let start:number|null = null;

    onMount(async ()=>{


        // Create scene, camera, and renderer
        const renderer = new THREE.WebGLRenderer({
            alpha: true,
            // antialias: true
        });
        renderer.outputColorSpace = THREE.LinearDisplayP3ColorSpace;
        renderer.setSize(element.clientWidth, element.clientHeight);
        element.appendChild(renderer.domElement)
        scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(30, element.clientWidth / element.clientHeight, 0.1, 100);

        const controls = new OrbitControls(camera, renderer.domElement)

        camera.position.set(0, 0, 4.3);
        controls.minPolarAngle = Math.PI/2 * 0.8;
        controls.maxPolarAngle = Math.PI/2 * 1.2;
        controls.update(0)
        controls.autoRotate = true;
        controls.autoRotateSpeed = 0.005;
        controls.enablePan = false;
        controls.enableZoom = false;
        controls.enableDamping = true;

    
        // // Animate the scene
        function animate(time:number) {
            start ??= time;
            const delta = time - start;
            start = time;
            requestAnimationFrame(animate);
            controls.update(delta)
            renderer.render(scene!, camera);
        }

        // window.addEventListener('resize', () => {
        //     camera.aspect = element.clientWidth / element.clientHeight;
        //     camera.updateProjectionMatrix();
        //     renderer.setSize(element.clientWidth, element.clientHeight);
        // });
    
        renderer.render(scene, camera);

        animate(0);
        loadSkin()
    })

    $: data, loadSkin();
</script>

<div bind:this={element} class="w-full h-full min-h-24 m-0"/>

<!-- <img src={`https://vzge.me/frontfull/832/${ "username" in data ?  data.username : data.uuid }.png`} alt={`player skin of ${ "username" in data ?  data.username : data.uuid }`}> -->
