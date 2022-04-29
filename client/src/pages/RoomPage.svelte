<script lang="ts">
	import api from '../api'
	import { roomStore } from '../store/room_store'
	import { onInterval } from '../common/svelte_utils'

	export let params

	let isNotFound = false

	// Interval to fetch room status
	onInterval(
		async () =>
			await api
				.fetchRoom(params.id)
				.then((_) => (isNotFound = false))
				.catch((error) => (isNotFound = true)),
		1000
	)
</script>

{#if $roomStore.id === params.id && !isNotFound}
	{$roomStore.expiration_timestamp}
{:else if isNotFound}
	Not Found
{/if}
