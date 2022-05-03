<script lang="ts">
	import { Splide, SplideSlide } from '@splidejs/svelte-splide'
	import '@splidejs/svelte-splide/css'
	import { createEventDispatcher } from 'svelte'
	import MemeCard from './MemeCard.svelte'

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
		autoWidth: true,
		pagination: false,
		focus: 'center',
	}}
	aria-label="Memes"
>
	{#each links as link}
		<SplideSlide>
			<MemeCard on:click={() => passCard(link)} {link} />
		</SplideSlide>
	{/each}
</Splide>
