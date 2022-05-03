<script>
	import { createEventDispatcher } from 'svelte'

	import { Card } from 'attractions'

	const dispatch = createEventDispatcher()

	export let link
</script>

<div class="meme-card-wrapper">
	<Card tight={true}
		><div on:click={(e) => dispatch('click', e)} class="meme-card">
			<img src={link} alt="meme" class="meme-img" />
		</div></Card
	>
</div>

<style>
	.meme-card-wrapper {
		width: 300px;
		margin: 0.3rem;
	}

	.meme-card {
		height: 400px;
		display: flex;
		align-items: center;
		padding: 0.3rem;
	}

	.meme-img {
		width: 100%;
	}
</style>
