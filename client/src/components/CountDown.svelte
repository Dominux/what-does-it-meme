<script lang="ts">
	import { onMount } from 'svelte'

	export let expireDate: Date

	let timeLeft = new Date(0)

	const pad = (n) => (n < 10 ? `0${n}` : n)

	onMount(() => {
		const interval = setInterval(() => {
			if (typeof expireDate === 'object') {
				const now = new Date()
				if (now < expireDate) {
					timeLeft = new Date(expireDate.getTime() - new Date().getTime())
				}
			}
		}, 1000)
		return () => clearInterval(interval)
	})
</script>

<div class="count-down">
	<span>{pad(timeLeft.getMinutes())}</span>:<span>{pad(timeLeft.getSeconds())}</span>
</div>

<style>
	.count-down {
		margin: 0.5rem;
		text-align: center;
		font-variant: tabular-nums;
		font-size: 48px;
	}
</style>
