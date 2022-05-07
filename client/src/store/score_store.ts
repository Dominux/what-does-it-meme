import { writable } from 'svelte/store'

const storedScore: { [key: string]: number } = JSON.parse(localStorage.getItem('score'))
export const scoreStore = writable(storedScore)
scoreStore.subscribe((value) => localStorage.setItem('score', JSON.stringify(value)))
