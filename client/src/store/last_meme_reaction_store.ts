import { writable } from 'svelte/store'

const storedLastMemeReaction = JSON.parse(localStorage.getItem('last_meme_reaction'))
export const lastMemeReactionStore = writable(storedLastMemeReaction)
lastMemeReactionStore.subscribe((value) =>
	localStorage.setItem('last_meme_reaction', JSON.stringify(value))
)
