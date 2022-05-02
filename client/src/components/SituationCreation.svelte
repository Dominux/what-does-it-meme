<script>
	import { Button, Loading, TextField } from 'attractions'
	import api from '../api'
	import { playerStore } from '../store/player_store'
	import { roomStore } from '../store/room_store'

	let situation = ''
	let isCreated = false
	let isLoading = false

	async function createSituation() {
		isLoading = true
		await api.createSituation(situation).catch((e) => alert(e))
		isCreated = true
		isLoading = false
	}
</script>

<br />
{#if $roomStore.isSituationCreator($playerStore.name)}
	<div class="form">
		<TextField placeholder="Some situation or funny thing" bind:value={situation} maxlength="255" />
		<Button disabled={isLoading || isCreated} filled on:click={createSituation}
			>{#if isLoading}<Loading />{:else}Create situation{/if}</Button
		>
	</div>
{/if}

<style>
	.form {
		margin: 3em 25% 0;
	}
</style>
