<script>
	import { Button, Loading, TextField } from 'attractions'
	import api from '../api'
	import { playerStore } from '../store/player'
	import { roomStore } from '../store/room_store'

	let situation = ''
	let isCreated = false
	let isLoading = false

	async function createSituation() {
		isLoading = true
		await api.createSituation(situation)
		isCreated = true
		isLoading = false
	}
</script>

{#if $roomStore.isSituationCreator($playerStore.name)}
	<div class="form">
		<TextField placeholder="Some situation or funny thing" bind:value={situation} />
		<Button disabled={isLoading || isCreated} filled on:click={createSituation}
			>{#if isLoading}<Loading />{:else}Join room{/if}</Button
		>
	</div>
{/if}

<style>
	.form {
		margin: 3em 25% 0;
	}
</style>
