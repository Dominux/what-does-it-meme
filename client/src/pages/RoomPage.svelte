<script lang="ts">
	import api from '../api'
	import { roomStore } from '../store/room_store'
	import { onInterval } from '../common/svelte_utils'
	import { RoomState, RoundState } from '../models/room'
	import NotStartedRoom from '../components/NotStartedRoom.svelte'
	import SituationCreation from '../components/SituationCreation.svelte'
	import ChoosingMeme from '../components/ChoosingMeme.svelte'
	import Voting from '../components/Voting.svelte'
	import CountDown from '../components/CountDown.svelte'
	import ShowingResults from '../components/ShowingResults.svelte'

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

	{#if $roomStore.round?.round_state !== RoundState.ShowingResults || $roomStore.state !== RoundState.Ended}
		<CountDown expireDate={$roomStore.expiration_timestamp} />
	{/if}

	{#if $roomStore.state === RoomState.NotStarted}
		<NotStartedRoom />
	{:else if $roomStore.round?.round_state === RoundState.SituationCreation}
		<SituationCreation />
	{:else if $roomStore.round?.round_state === RoundState.ChoosingMemes}
		<ChoosingMeme />
	{:else if $roomStore.round?.round_state === RoundState.Voting}
		<Voting />
	{:else if $roomStore.round?.round_state === RoundState.ShowingResults}
		<ShowingResults />
	{/if}
{:else if isNotFound}
	<!-- Room not found -->
	Not Found
{:else}
	...loading
{/if}
