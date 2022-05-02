import { writable } from 'svelte/store'
import type { Player } from '../models/room'

const storedPlayer: Player = JSON.parse(localStorage.getItem('player'))
export const playerStore = writable(storedPlayer)
playerStore.subscribe((value) => localStorage.setItem('player', JSON.stringify(value)))
