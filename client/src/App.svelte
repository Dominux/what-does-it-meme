<script lang="ts">
	import Home from './pages/Home.svelte'
	import Router, { location, link } from 'svelte-spa-router'
	import Room from './pages/Room.svelte'
</script>

<Router
	routes={{
		'/': Home,
		'/rooms/new_room': Room,
	}}
/>
