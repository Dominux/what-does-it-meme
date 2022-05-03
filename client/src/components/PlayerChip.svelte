<script lang="ts">
	import { Chip } from 'attractions'
	export let name: string
</script>

<div class="player-chip">
	<Chip>{name}</Chip>
</div>

<style>
	.player-chip {
		margin: 0.1rem;
		height: fit-content;
		width: fit-content;
	}
</style>
