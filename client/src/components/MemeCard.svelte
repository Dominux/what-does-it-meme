<script>
	import { createEventDispatcher } from 'svelte'

	import { Card } from 'attractions'

	const dispatch = createEventDispatcher()

	export let link
</script>

<div class="meme-card-wrapper">
	<Card
		><div on:click={(e) => dispatch('click', e)} class="meme-card">
			<img src={link} alt="meme" class="meme-img" />
		</div></Card
	>
</div>

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
