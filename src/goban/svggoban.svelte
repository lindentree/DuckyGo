<script lang="ts">
    import { onMount } from 'svelte';
    import board from '../assets/goban.svg';
    import Grid from './Grid.svelte';
	
	const size = 600;
	let canvas;
  
	onMount(async () => {
		// const ctx = canvas.getContext('2d');
		// const image = new Image();
		// image.src = board;
		// image.onload = () => {
		// 	ctx.fillStyle = ctx.createPattern(image, 'repeat'); 
		// 	ctx.fillRect(0, 0, canvas.width, canvas.height)
		// }; 
	})
    
</script>


<main>
    <div id="goban" style="height:100vh; width:auto; background-image: url('{board}'); background-size: cover">
    <canvas></canvas>
    <Grid></Grid>
    </div>
       

</main>
  
  <style>
  </style>