<script lang="ts">
	import { fly, fade, scale } from 'svelte/transition'

	import { Button, Headline, Subhead } from 'attractions'
	import { push } from 'svelte-spa-router'
	import api from '../api'

	async function createRoom() {
		try {
			const room = await api.createRoom()
			push(`/rooms/${room.id}`)
		} catch (error) {
			alert(error)
		}
	}
</script>

<div out:scale>
	<div class="header" in:fly={{ y: 250, duration: 1000 }}>
		<Headline>What Does it Meme</Headline>
		<Subhead>THe best game EVER!!!</Subhead>
	</div>

	<div class="btn" in:fade={{ duration: 800, delay: 1700 }}>
		<Button danger on:click={createRoom}><div class="btn-text">Click to create room</div></Button>
	</div>
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
