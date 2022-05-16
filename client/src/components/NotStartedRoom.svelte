<script>
	import { Button } from 'attractions'

	import JoinRoomForm from './JoinRoomForm.svelte'
	import PlayersProgress from './PlayersProgress.svelte'
	import { roomStore } from '../store/room_store'
	import { playerStore } from '../store/player_store'
	import api from '../api'

	const location = window.location.href
	const minPlayers = 3

	function copyLocationToClipboard() {
		navigator.clipboard.writeText(location)
	}
</script>

<div class="link">
	<Button on:click={copyLocationToClipboard} filled small>Copy</Button>
	<a href={location}>link to the room</a>
</div>

{#if !$playerStore}
	<JoinRoomForm />
{:else if $roomStore.players?.length >= minPlayers}
	<Button outline on:click={api.startGame}>Start Game</Button>
{:else}
	<Button disabled outline>Minimum {minPlayers} players needed to start game</Button>
{/if}

<h3>Players:</h3>
<PlayersProgress
	players={$roomStore.players?.map((p) => {
		return { name: p.name, isReady: true }
	})}
/>

<style>
	.link {
		display: flex;
		flex-wrap: nowrap;
		flex-direction: row;
		align-items: baseline;
		gap: 0.3em;
	}
</style>
