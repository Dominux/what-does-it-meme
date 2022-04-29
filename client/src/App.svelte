<script lang="ts">
	import HomePage from './pages/HomePage.svelte'
	import Router from 'svelte-spa-router'
	import RoomPage from './pages/RoomPage.svelte'
</script>

<Router
	routes={{
		'/': HomePage,
		'/rooms/:id': RoomPage,
	}}
/>
