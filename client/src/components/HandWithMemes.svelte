<script lang="ts">
	import { Card } from 'attractions'
	import { Splide, SplideSlide } from '@splidejs/svelte-splide'
	import '@splidejs/svelte-splide/css'
	import { createEventDispatcher } from 'svelte'

	const dispatch = createEventDispatcher()

	export let links: Array<string>

	function passCard(link: string) {
		dispatch('passCard', {
			link,
		})
	}
</script>

<Splide
	options={{
		drag: 'free',
		snap: true,
		perPage: 3,
		// gap: 5,
		autoWidth: false,
		pagination: false,
		focus: 'center',
	}}
	aria-label="Memes"
>
	{#each links as link, i}
		<SplideSlide>
			<div class="meme-card-wrapper">
				<Card
					><div on:click={() => passCard(link)} class="meme-card">
						<img src={link} alt="meme" class="meme-img" />
					</div></Card
				>
			</div>
		</SplideSlide>
	{/each}
</Splide>

<style>
	.meme-card-wrapper {
		margin: 20px;
		transition: all 0.25s linear;
	}
	.meme-card-wrapper:hover {
		margin: 0;
	}

	.meme-card {
		height: 400px;
	}

	.meme-img {
		width: 100%;
		height: auto;
		max-height: 100%;
		vertical-align: middle;
	}
</style>
