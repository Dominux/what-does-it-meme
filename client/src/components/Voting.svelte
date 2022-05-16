<script lang="ts">
	import { get } from 'svelte/store'
	import { Loading } from 'attractions'

	import api from '../api'
	import { lastMemeReactionStore } from '../store/last_meme_reaction_store'
	import { playerStore } from '../store/player_store'
	import { roomStore } from '../store/room_store'

	import MemeCard from './MemeCard.svelte'
	import PlayersProgress from './PlayersProgress.svelte'

	let isVoted = false
	let isLoading = false

	async function vote(meme_id: string) {
		isLoading = true
		await api.vote(meme_id).catch((e) => alert(e))
		isVoted = true
		isLoading = false
	}

	$: pendingPlayers = $roomStore.players
		.filter((p) => p.name !== $roomStore.round?.situation_creator_name)
		.map((p) => {
			return { name: p.name, isReady: $roomStore.round?.reacted_players_names?.includes(p.name) }
		})

	$: memes = $roomStore.round?.memes.filter((meme) => meme.link !== get(lastMemeReactionStore))
</script>

<br />

<h3>
	{$roomStore.round?.situation}
</h3>

{#if !$roomStore.isSituationCreator($playerStore.name) && !isVoted}
	{#if isLoading}
		<Loading />
	{:else}
		<div class="memes-to-vote">
			{#each memes as meme}
				<MemeCard link={meme.link} on:click={() => vote(meme.meme_id)} />
			{/each}
		</div>
	{/if}
{:else}
	Wait for others to vote
{/if}

<br />

<PlayersProgress players={pendingPlayers} />

<style>
	.memes-to-vote {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
	}
</style>
