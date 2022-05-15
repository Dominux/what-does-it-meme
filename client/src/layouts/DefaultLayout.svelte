<script lang="ts">
	import { getActualLocation } from '../common/location_utils'
	import { RoomState } from '../models/room'

	import { playerStore } from '../store/player_store'
	import { roomStore } from '../store/room_store'
</script>

<main>
	<header>
		<a href={getActualLocation('')}><h4>Home</h4></a>

		{#if $playerStore && $roomStore.state !== RoomState.Ended}
			<a href={getActualLocation(`/#/rooms/${$roomStore.id}`)} label="Come back to the game"
				><h3 class="name-block">{$playerStore.name}</h3></a
			>
		{/if}
	</header>

	<slot />
</main>

<style>
	main {
		max-width: 1080px;
		margin: 0 auto;
	}

	header {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}

	h4 {
		margin: 0.5rem;
	}

	.name-block {
		position: absolute;
		top: 0;
		right: 0;
		background-color: aqua;
		padding: 1rem 2rem;
		margin: 0;
		text-decoration: none;
		color: black;
	}
</style>
