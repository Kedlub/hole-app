<script lang="ts">
	import Titlebar from "./lib/Titlebar.svelte";

	let showHole = false;
</script>

<div class="main-container">
	<Titlebar />
	<main class="container" class:hole={showHole}>
		<div>
			<h1>Totally normal app</h1>
			<p>
				This is a totally normal app. It does nothing special. It's just a
				regular app.
			</p>
		</div>
		<div class="container-grow">
			{#if !showHole}
				<button on:click={() => (showHole = !showHole)}>
					This is just a regular button
				</button>
			{/if}
		</div>
	</main>
</div>