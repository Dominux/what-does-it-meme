<script>
	import { location } from 'svelte-spa-router'
	import { Button } from 'attractions'

	import JoinRoomForm from './JoinRoomForm.svelte'
	import { roomStore } from '../store/room_store'
	import { playerStore } from '../store/player_store'
	import api from '../api'

	const minPlayers = process.env.PLAYERS_MINIMUM | 3
</script>

players: {$roomStore.players?.map((p) => p.name)}

<div>
	<!-- TODO: make it copiable by clicking to the copy icon -->
	Copy <a href={`/#${$location}`}>link to the room</a>
</div>

{#if !$playerStore}
	<JoinRoomForm />
{:else if $roomStore.players?.length >= minPlayers}
	<Button outline on:click={api.startGame}>Start Game</Button>
{:else}
	<Button disabled outline>Minimum {minPlayers} players needed to start game</Button>
{/if}
