<script lang="ts">
	import { Loading } from 'attractions'

	import api from '../api'
	import { lastMemeReactionStore } from '../store/last_meme_reaction_store'

	import { playerStore } from '../store/player_store'

	import { roomStore } from '../store/room_store'
	import HandWithMemes from './HandWithMemes.svelte'
	import PlayersProgress from './PlayersProgress.svelte'

	let isCreated = false
	let isLoading = false

	$: pendingPlayers = $roomStore.players
		.filter((p) => p.name !== $roomStore.round?.situation_creator_name)
		.map((p) => {
			return { name: p.name, isReady: $roomStore.round?.reacted_players_names.includes(p.name) }
		})

	async function reactWithMeme(e) {
		isLoading = true
		await api.reactWithMeme(e.detail.link).catch((e) => alert(e))
		isCreated = true
		isLoading = false

		lastMemeReactionStore.set(e.detail.link)
	}
</script>

<br />

<h3>
	{$roomStore.round?.situation || 'Situation is not created'}
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
<PlayersProgress players={pendingPlayers} />
