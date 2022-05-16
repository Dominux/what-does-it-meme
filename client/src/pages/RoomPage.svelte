<script lang="ts">
	import { fade, fly } from 'svelte/transition'

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

<div in:fade={{ delay: 500 }}>
	{#if $roomStore?.id === params.id && !isNotFound}
		<!-- We are inside right room -->

		{#if $roomStore.round?.round_state !== RoundState.ShowingResults || $roomStore.state !== RoomState.Ended}
			<CountDown expireDate={$roomStore.expiration_timestamp} />
		{/if}

		{#if $roomStore.state === RoomState.NotStarted}
			<div transition:fly>
				<NotStartedRoom />
			</div>
		{:else if $roomStore.round?.round_state === RoundState.SituationCreation}
			<div in:fade={{ delay: 500 }} out:fly>
				<SituationCreation />
			</div>
		{:else if $roomStore.round?.round_state === RoundState.ChoosingMemes}
			<div in:fade={{ delay: 500 }} out:fly>
				<ChoosingMeme />
			</div>
		{:else if $roomStore.round?.round_state === RoundState.Voting}
			<div in:fade={{ delay: 500 }} out:fly>
				<Voting />
			</div>
		{:else if $roomStore.round?.round_state === RoundState.ShowingResults}
			<div in:fade={{ delay: 500 }} out:fly>
				<ShowingResults />
			</div>
		{/if}
	{:else if isNotFound}
		<!-- Room not found -->
		Not Found
	{:else}
		...loading
	{/if}
</div>
