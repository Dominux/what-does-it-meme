<script lang="ts">
	import api from '../api'
	import { roomStore } from '../store/room_store'
	import { onInterval } from '../common/svelte_utils'
	import { RoomState } from '../models/room'
	import NotStartedRoom from '../components/NotStartedRoom.svelte'
	import { playerStore } from '../store/player'

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
	{/if}
{:else if isNotFound}
	<!-- Room not found -->
	Not Found
{:else}
	...loading
{/if}
