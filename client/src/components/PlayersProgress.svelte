<script lang="ts">
	import type { PendingPlayer } from '../models/room'

	import PlayerChip from './PlayerChip.svelte'

	export let players: Array<PendingPlayer>
</script>

<ul class="players-progress">
	{#each players as { name, isReady }}
		<li>
			<div class:player-ready={isReady}>
				<PlayerChip {name} />
			</div>
		</li>
	{/each}
</ul>

<style>
	.players-progress {
		list-style-type: none;
		padding: 0;
	}

	.player-ready {
		width: fit-content;
		margin-left: auto;
		transition: all 0.25s linear;
	}
</style>
