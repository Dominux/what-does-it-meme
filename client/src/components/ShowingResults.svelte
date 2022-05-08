<script lang="ts">
	import { H1, H2 } from 'attractions'
	import { onMount } from 'svelte'
	import api from '../api'
	import { roomStore } from '../store/room_store'
	import MemeCard from './MemeCard.svelte'
	import PlayerChip from './PlayerChip.svelte'

	const secsToShowResults = 10
	let toShowResults = true
	let scorePlayers = []

	onMount(async () => {
		setTimeout(() => (toShowResults = false), secsToShowResults * 1000)
		scorePlayers = Object.entries(await api.getScore()).sort((a, b) => b[1] - a[1])
	})
</script>

<div class="header">
	<H1>Results</H1>
</div>

{#if toShowResults}
	<!-- Results -->
	<div class="results">
		{#each $roomStore?.round?.memes || [] as meme}
			<div>
				<div class="meme-card-result meme-card-voters">
					{#each meme.voters_names as name}
						<PlayerChip {name} />
					{/each}
				</div>
				<div class="meme-card-result meme-card-authorname">
					<H2>{meme.author_name}</H2>
				</div>
				<MemeCard link={meme.link} />
			</div>
		{/each}
	</div>
{:else}
	<!-- Score -->
	<div class="rating-wrapper">
		{#each scorePlayers as scorePlayer}
			<PlayerChip name={scorePlayer[0]} />
			{scorePlayer[1]}
		{/each}
	</div>
{/if}

<style>
	.header {
		text-align: center;
	}

	.results {
		display: flex;
		flex-wrap: wrap;
		gap: 0.3rem;
	}

	.meme-card-result {
		z-index: 99;
		position: absolute;
		width: 300px;
		margin: 0.5rem;
	}

	.meme-card-voters {
		height: 400px;
		display: flex;
		flex-wrap: wrap;
		align-content: flex-start;
	}

	.meme-card-authorname {
		margin-top: 340px;
		text-align: center;
	}

	.rating-wrapper {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		justify-items: center;
		align-items: center;
		margin: 3rem 25%;
		font-size: 150%;
	}
</style>
