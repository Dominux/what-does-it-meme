<script lang="ts">
	import { Loading } from 'attractions'
	import { get } from 'svelte/store'

	import api from '../api'
	import { lastMemeReactionStore } from '../store/last_meme_reaction_store'

	import { playerStore } from '../store/player_store'

	import { roomStore } from '../store/room_store'
	import MemeCard from './MemeCard.svelte'

	let isVoted = false
	let isLoading = false

	async function vote(meme_id: string) {
		isLoading = true
		await api.vote(meme_id)
		isVoted = true
		isLoading = false
	}

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
		<ul class="memes-to-vote">
			{#each memes as meme}
				<li>
					<MemeCard link={meme.link} on:click={() => vote(meme.meme_id)} />
				</li>
			{/each}
		</ul>
	{/if}
{:else}
	Wait for others to vote
{/if}

<br />
voted_players: {$roomStore.round?.reacted_players_names}

<style>
	.memes-to-vote {
		list-style-type: none;
	}
</style>
