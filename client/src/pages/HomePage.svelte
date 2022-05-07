<script lang="ts">
	import { Button, Chip, Headline, Subhead } from 'attractions'
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

<div class="header">
  <div class="beta-sht">
    <Chip>Beta</Chip>
  </div>
	<Headline>What Does it Meme</Headline>
	<Subhead>THe best game EVER!!!</Subhead>
</div>

<div class="btn">
	<Button danger on:click={createRoom}><div class="btn-text">Click to create room</div></Button>
</div>

<style>
  .beta-sht {
    width: fit-content;
    font-size: 50%;
    z-index: 99;
    position: absolute;
    /* margin-right: 20%; */
    margin-left: 83%;
  }

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
