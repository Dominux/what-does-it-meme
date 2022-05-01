<script lang="ts">
	import Router from 'svelte-spa-router'

	import HomePage from './pages/HomePage.svelte'
	import RoomPage from './pages/RoomPage.svelte'
	import DefaultLayout from './layouts/DefaultLayout.svelte'
	import HandWithMemes from './components/HandWithMemes.svelte'
</script>

<HandWithMemes />

<!-- <DefaultLayout>
	<Router
		routes={{
			'/': HomePage,
			'/rooms/:id': RoomPage,
		}}
	/>
</DefaultLayout> -->
