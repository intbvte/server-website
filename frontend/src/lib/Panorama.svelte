<script lang="ts">
	import { onMount } from 'svelte';
    import * as THREE from 'three';

    let element:HTMLDivElement;

    onMount(()=>{

        // Create scene, camera, and renderer
        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(75, element.clientWidth / element.clientHeight, 0.1, 1);
        const renderer = new THREE.WebGLRenderer();
        renderer.outputColorSpace = THREE.LinearDisplayP3ColorSpace;
        renderer.setSize(element.clientWidth, element.clientHeight);
        element.appendChild(renderer.domElement)
    
        // Create cube geometry for the panorama
        const geometry = new THREE.BoxGeometry(-1, 1, 1);
        const loader = new THREE.TextureLoader();

    
        // Load each face of the cube with a different image
        const materials = [
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_1.webp') }), // Right
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_3.webp') }), // Left
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_4.webp') }), // Top
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_5.webp') }), // Bottom
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_0.webp') }), // Front
            new THREE.MeshBasicMaterial({ map: loader.load('/background/panorama_2.webp') })  // Back
        ];
    
        // Create mesh with the cube geometry and texture materials
        const cube = new THREE.Mesh(geometry, materials);
        scene.add(cube);
        cube.rotation.y = 4.2;
    
        // Set camera position inside the cube
        camera.position.set(0, 0, 0);

    
        // Animate the scene
        function animate() {
            requestAnimationFrame(animate);
            cube.rotation.y += 0.0005;
            renderer.render(scene, camera);
        }

        window.addEventListener('resize', () => {
            camera.aspect = element.clientWidth / element.clientHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(element.clientWidth, element.clientHeight);
        });
    
        animate();
    })
</script>

<div bind:this={element} class="w-full h-full m-0"/>