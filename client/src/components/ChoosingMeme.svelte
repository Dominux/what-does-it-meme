<script lang="ts">
	import { Loading } from 'attractions'

	import api from '../api'

	import { playerStore } from '../store/player_store'

	import { roomStore } from '../store/room_store'
	import HandWithMemes from './HandWithMemes.svelte'

	let isCreated = false
	let isLoading = false

	async function reactWithMeme(e) {
		isLoading = true
		await api.reactWithMeme(e.detail.link)
		isCreated = true
		isLoading = false
	}
</script>

<br />

<h3>
	{$roomStore.round?.situation}
</h3>

{#if !$roomStore.isSituationCreator($playerStore.name) && !isCreated}
	{#if isLoading}
		<Loading />
	{:else}
		<HandWithMemes links={$playerStore.memes_in_hand} on:passCard={reactWithMeme} />
	{/if}
{:else}
	Wait for others to react to the situation
{/if}

<br />
reacted_players_names: {$roomStore.round?.reacted_players_names}
