<script lang="ts">
	import { Button, Loading, TextField } from 'attractions'
	import api from '../api'

	let playerName = ''
	let isLoading = false
	let isCreated = false

	$: isDisabled = isCreated || isLoading

	async function joinRoom() {
		isLoading = true
		const player = await api.joinRoom(playerName).catch((error) => alert(error))
		playerName = ''
		isLoading = false

		if (player) isCreated = true
	}

	async function handleOnKeyDown(event: CustomEvent) {
		if (!isDisabled && event.detail.nativeEvent.key === 'Enter') await joinRoom()
	}
</script>

{#if !isDisabled}
	<div class="form">
		<TextField
			maxlength={16}
			label="Player name"
			placeholder="Ur mom"
			bind:value={playerName}
			outline
			on:keydown={handleOnKeyDown}
		/>
		<Button disabled={isDisabled || playerName.length < 1} filled on:click={joinRoom}>
			{#if isLoading}
				<Loading />
			{:else}
				Join room
			{/if}
		</Button>
	</div>
{/if}

<style>
	:global(.form) {
		margin: 1em 8%;
	}
</style>
