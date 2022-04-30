<script>
	import { Button, Loading, TextField } from 'attractions'
	import api from '../api'

	let playerName = ''
	let isLoading = false

	async function joinRoom() {
		isLoading = true
		await api.joinRoom(playerName).catch((error) => alert(error))
		playerName = ''
		isLoading = false
	}
</script>

<div class="form">
	<TextField label="Player name" placeholder="Ur mom" bind:value={playerName} outline />
	<Button disabled={isLoading} filled class="icon" on:click={joinRoom}
		>{#if isLoading}<Loading />{:else}Join room{/if}</Button
	>
</div>

<style>
	.form {
		margin: 3em 25% 0;
	}
</style>
