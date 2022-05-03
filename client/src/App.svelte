<script lang="ts">
	import Router from 'svelte-spa-router'

	import HomePage from './pages/HomePage.svelte'
	import RoomPage from './pages/RoomPage.svelte'
	import DefaultLayout from './layouts/DefaultLayout.svelte'
	import HandWithMemes from './components/HandWithMemes.svelte'
</script>

<DefaultLayout>
	<Router
		routes={{
			'/': HomePage,
			'/rooms/:id': RoomPage,
		}}
	/>
</DefaultLayout>

<!-- <HandWithMemes
	links={[
		'https://i.kym-cdn.com/entries/icons/original/000/005/099/bird-business-cat.jpg',
		'https://i.kym-cdn.com/entries/icons/original/000/005/868/tumblr_lj314fRiAg1qe9semo1_500.jpg',
		'https://i.kym-cdn.com/entries/icons/original/000/017/788/gotem.PNG',
		'https://i.kym-cdn.com/entries/icons/original/000/025/771/215.jpg',
		'https://i.kym-cdn.com/entries/icons/original/000/021/498/cover5.jpg',
	]}
/> -->
