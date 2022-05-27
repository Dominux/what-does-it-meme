<script lang="ts">
	import { Button, Loading, TextField } from 'attractions'
	import api from '../api'
	import { playerStore } from '../store/player_store'
	import { roomStore } from '../store/room_store'

	let situation = ''
	let isCreated = false
	let isLoading = false

	$: isDisabled = isCreated || isLoading

	async function createSituation() {
		isLoading = true
		await api.createSituation(situation).catch((e) => alert(e))
		isCreated = true
		isLoading = false
	}

	async function handleOnKeyDown(event: CustomEvent) {
		if (!isDisabled && event.detail.nativeEvent.key === 'Enter') await createSituation()
	}
</script>

{#if $roomStore.isSituationCreator($playerStore.name)}
	{#if !isDisabled}
		<div class="form">
			<TextField
				placeholder="Some situation or funny thing"
				bind:value={situation}
				on:keydown={handleOnKeyDown}
				maxlength={255}
			/>
			<Button disabled={isLoading || isCreated} filled on:click={createSituation}
				>{#if isLoading}<Loading />{:else}Create situation{/if}</Button
			>
		</div>
	{/if}
{:else}
	Wait for {$roomStore.round?.situation_creator_name} to create situation
{/if}
