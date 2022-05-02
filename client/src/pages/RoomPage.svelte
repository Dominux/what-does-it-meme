<script lang="ts">
	import api from '../api'
	import { roomStore } from '../store/room_store'
	import { onInterval } from '../common/svelte_utils'
	import { RoomState, RoundState } from '../models/room'
	import NotStartedRoom from '../components/NotStartedRoom.svelte'
	import { playerStore } from '../store/player_store'
	import SituationCreation from '../components/SituationCreation.svelte'
	import ChoosingMeme from '../components/ChoosingMeme.svelte'

	export let params

	let isNotFound = false

	// Interval to fetch room and round status
	onInterval(
		async () =>
			await api
				.fetchRoom(params.id)
				.then((_) => (isNotFound = false))
				.catch((error) => (isNotFound = true)),
		1500
	)
</script>

{#if $roomStore?.id === params.id && !isNotFound}
	<!-- We are inside right room -->

	<h1>Current player: {$playerStore?.id}</h1>

	{$roomStore.expiration_timestamp}

	{#if $roomStore.state === RoomState.NotStarted}
		<NotStartedRoom />
	{:else if $roomStore.round?.round_state === RoundState.SituationCreation}
		<SituationCreation />
	{:else if $roomStore.round?.round_state === RoundState.ChoosingMemes}
		<ChoosingMeme />
	{:else if $roomStore.round?.round_state === RoundState.Voting}
		voting
	{/if}
{:else if isNotFound}
	<!-- Room not found -->
	Not Found
{:else}
	...loading
{/if}
