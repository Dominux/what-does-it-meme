<script lang="ts">
	import { Card } from 'attractions'
	import { afterUpdate, onMount } from 'svelte'

	let links = [
		'https://cdnb.artstation.com/p/assets/images/images/007/553/893/large/jun-you-si-color1.jpg?1506952392',
		'https://cdnb.artstation.com/p/assets/images/images/007/670/281/large/erikas-perl-supper.jpg?1507738456',
		'https://cdnb.artstation.com/p/assets/images/images/007/553/893/large/jun-you-si-color1.jpg?1506952392',
		'https://cdnb.artstation.com/p/assets/images/images/007/670/281/large/erikas-perl-supper.jpg?1507738456',
		'https://cdnb.artstation.com/p/assets/images/images/007/553/893/large/jun-you-si-color1.jpg?1506952392',
		'https://cdnb.artstation.com/p/assets/images/images/007/670/281/large/erikas-perl-supper.jpg?1507738456',
		'https://cdnb.artstation.com/p/assets/images/images/007/553/893/large/jun-you-si-color1.jpg?1506952392',
		'https://cdnb.artstation.com/p/assets/images/images/007/670/281/large/erikas-perl-supper.jpg?1507738456',
	]

	let carousel: HTMLDivElement

	// $: {
	// 	console.log(x, maxX)
	// }

	window.addEventListener('scroll', () => {
		console.log('lol')

		const { scrollWidth } = document.documentElement

		console.log(window.scrollX, window.innerWidth, scrollWidth)

		const scrollPercentage = (window.scrollX + 1) / (scrollWidth - window.innerWidth + 1)
		// console.log(scrollPercentage)

		const position = Math.ceil(scrollPercentage * links.length)

		carousel.style.setProperty('--position', position.toString())
	})
</script>

<div class="scroll-area">
	&#8291;
	<div bind:this={carousel} id="carousel">
		{#each links as link, i}
			<div class="item" style="--offset: {i + 1}">
				<Card class="meme-card"
					><div class="meme-card">
						<img src={link} alt="meme" class="meme-img" />
					</div></Card
				>
			</div>
		{/each}
	</div>
</div>

<style>
	/* body {
		height: 600px;
		margin: 0;
		scrollbar-width: none;
	}

	body::-webkit-scrollbar {
		display: none;
	} */

	.scroll-area {
		width: 200vw;
	}

	#carousel {
		position: fixed;
		width: 100vw;
		height: 500px;
		display: flex;
		justify-self: center;
		align-items: center;
		justify-content: center;
		transform-style: preserve-3d;
		perspective: 1000px;
		/* --items: 5; */
		--middle: 3;
		--position: 1;
		pointer-events: none;
	}

	div.item {
		position: absolute;
		width: 70%;
		--r: calc(var(--position) - var(--offset));
		--abs: max(calc(var(--r) * -1), var(--r));
		transition: all 0.25s linear;
		transform: rotateY(calc(-10deg * var(--r))) translateX(calc(-300px * var(--r)));
		z-index: calc((var(--position) - var(--abs)));
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
