<script lang="ts">
	import { Button, Headline, Subhead } from 'attractions'
	import { push } from 'svelte-spa-router'

	async function createRoom() {
		push('/rooms/new_room')
	}
</script>

<div class="header">
	<Headline>What Does it Meme</Headline>
	<Subhead>THe best game EVER!!!</Subhead>
</div>

<div class="btn">
	<Button danger on:click={createRoom}><div class="btn-text">Click to create room</div></Button>
</div>

<style>
	.header {
		font-size: 220%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		min-height: 80vh;
		text-align: center;
	}

	.btn {
		display: flex;
		justify-content: center;
		font-size: large;
	}
	.btn-text {
		font-size: 220%;
	}
</style>
